use toml::Table;
use wasmtime::{AsContextMut, TypedFunc, WasmParams, WasmResults};

use crate::{helpers::serde::transcode_toml_to_message_pack, prelude::*, wasm::memory::Segment};

pub struct Instance(wasmtime::Instance);

impl From<wasmtime::Instance> for Instance {
    fn from(inner: wasmtime::Instance) -> Self {
        Self(inner)
    }
}

impl Instance {
    #[allow(clippy::future_not_send)]
    pub fn read_bytes<D: Send>(
        &self,
        mut store: impl AsContextMut<Data = D>,
        segment: Segment,
    ) -> Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(segment.size);
        if !buffer.is_empty() {
            self.0
                .get_memory(&mut store, "memory")
                .ok_or_else(|| anyhow!("module  does not export `memory`"))?
                .read(&store, segment.offset, &mut buffer)
                .context("failed to read the instance's memory")?;
        }
        Ok(buffer)
    }

    /// Write the byte string into the instance's memory.
    ///
    /// Byte buffer is automatically allocated in the instance's memory.
    /// The module **must** export `memory` and `alloc()` function.
    ///
    /// # Returns
    ///
    /// Buffer offset.
    #[allow(clippy::future_not_send)]
    pub async fn write_bytes<D: Send>(
        &mut self,
        mut store: impl AsContextMut<Data = D>,
        value: &[u8],
    ) -> Result<Segment> {
        let offset =
            self.alloc(&mut store, value.len()).await.context("failed to allocate memory")?;
        self.0
            .get_memory(&mut store, "memory")
            .ok_or_else(|| anyhow!("module does not export `memory`"))?
            .write(&mut store, offset, value)
            .context("failed to write the buffer into the instance's memory")?;
        Ok(Segment::new(offset, value.len()))
    }

    /// Allocate a buffer of `size` bytes in the instance's memory.
    ///
    /// # Returns
    ///
    /// Offset of the allocated buffer.
    #[allow(clippy::future_not_send)]
    async fn alloc<S: Send>(
        &self,
        store: impl AsContextMut<Data = S>,
        size: usize,
    ) -> Result<usize> {
        let offset: u32 = self.call_typed_func_async(store, "alloc", u32::try_from(size)?).await?;
        Ok(usize::try_from(offset)?)
    }

    #[allow(clippy::future_not_send)]
    async fn call_typed_func_async<S: Send, P: WasmParams, R: WasmResults>(
        &self,
        mut store: impl AsContextMut<Data = S>,
        name: &str,
        params: P,
    ) -> Result<R> {
        let function: TypedFunc<P, R> = self
            .0
            .get_typed_func(&mut store, name)
            .with_context(|| format!("failed to get function `{name}`",))?;
        function
            .call_async(&mut store, params)
            .await
            .with_context(|| format!("failed to call function `{name}` from module",))
    }
}

/// Companion's service connection via WASM module instance.
pub struct Connection(Instance);

impl From<Instance> for Connection {
    fn from(instance: Instance) -> Self {
        Self(instance)
    }
}

impl Connection {
    /// Call the module's `init(i32, i32) -> (i32, i32)`.
    ///
    /// The `init()` must accept a MessagePack-encoded byte string (address and length),
    /// and return a Protocol Buffers-encoded state byte string. Companion will allocate and write
    /// the string to the memory prior to calling the `init()`.
    ///
    /// Return length may be equal to `0` – in that case, Companion does not need access to the memory.
    ///
    /// # Returns
    ///
    /// Byte string, returned by the `init()`.
    #[allow(clippy::future_not_send)]
    pub async fn call_init_async<D: Send>(
        &mut self,
        mut store: impl AsContextMut<Data = D>,
        settings: Table,
    ) -> Result<Vec<u8>> {
        let settings = transcode_toml_to_message_pack(settings)?;
        let segment = self.0.write_bytes(&mut store, &settings).await?;
        let (offset, size): (u32, u32) = self
            .0
            .call_typed_func_async(&mut store, "init", segment.as_tuple_u32()?)
            .await?;
        self.0.read_bytes(&mut store, Segment::from_u32(offset, size)?)
    }
}
