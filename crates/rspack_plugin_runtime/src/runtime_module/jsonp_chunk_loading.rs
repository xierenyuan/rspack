use crate::runtime_module::utils::{get_initial_chunk_ids, stringify_chunks};
use hashbrown::HashSet;
use rspack_core::{
  rspack_sources::{BoxSource, ConcatSource, RawSource, SourceExt},
  runtime_globals, ChunkUkey, Compilation, RuntimeModule,
};

#[derive(Debug, Default)]
pub struct JsonpChunkLoadingRuntimeModule {
  chunk: Option<ChunkUkey>,
  runtime_requirements: HashSet<String>,
}

impl JsonpChunkLoadingRuntimeModule {
  pub fn new(runtime_requirements: HashSet<String>) -> Self {
    Self {
      chunk: None,
      runtime_requirements,
    }
  }
}

impl RuntimeModule for JsonpChunkLoadingRuntimeModule {
  fn identifier(&self) -> &str {
    "webpack/runtime/jsonp_chunk_loading"
  }

  fn generate(&self, compilation: &Compilation) -> BoxSource {
    // let condition_map = compilation.chunk_graph.get_chunk_condition_map(
    //   &chunk_ukey,
    //   &compilation.chunk_by_ukey,
    //   &compilation.chunk_group_by_ukey,
    //   &compilation.module_graph,
    //   chunk_hash_js,
    // );
    let initial_chunks = get_initial_chunk_ids(self.chunk, compilation);
    let mut source = ConcatSource::default();
    source.add(RawSource::from(
      include_str!("runtime/jsonp_chunk_loading.js")
        .to_string()
        .replace("INSTALLED_CHUNKS", &stringify_chunks(&initial_chunks, 0))
        // TODO
        .replace("JS_MATCHER", "chunkId"),
    ));

    if self
      .runtime_requirements
      .contains(runtime_globals::ON_CHUNKS_LOADED)
    {
      source.add(RawSource::from(
        include_str!("runtime/jsonp_chunk_loading_with_on_chunk_load.js").to_string(),
      ));
    }

    source.boxed()
  }

  fn attach(&mut self, chunk: ChunkUkey) {
    self.chunk = Some(chunk);
  }
}