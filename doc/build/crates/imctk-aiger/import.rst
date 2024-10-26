==============
``mod import``
==============


.. rust:module:: imctk_aiger::import
   :index: 0
   :vis: pub


   .. rust:use:: imctk_aiger::import
      :used_name: self


   .. rust:use:: std::io
      :used_name: io


   .. rust:use:: flussab_aiger::aig::OrderedAig
      :used_name: OrderedAig


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: imctk_ids::id_vec::IdVec
      :used_name: IdVec


   .. rust:use:: imctk_ir::env::Env
      :used_name: Env


   .. rust:use:: imctk_ir::node::generic::TermNode
      :used_name: TermNode


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rust:use:: imctk_ir::node::fine::circuit::And
      :used_name: And


   .. rust:use:: imctk_ir::env::EnvWrapper
      :used_name: EnvWrapper


   .. rust:use:: imctk_ir::node::fine::circuit::Input
      :used_name: Input


   .. rust:use:: imctk_ir::node::builder::NodeBuilder
      :used_name: NodeBuilder


   .. rust:use:: imctk_ir::node::fine::circuit::Reg
      :used_name: Reg


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_ir::node::fine::circuit::SteadyInput
      :used_name: SteadyInput


   .. rubric:: Types


   .. rust:type:: imctk_aiger::import::AigerLit
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"AigerLit"}]

      Type alias used to indicate which literals use the AIGER numbering as opposed to the target
      numbering.
      
      Note that this is a type alias and thus does not provide any type checking.

   .. rust:type:: imctk_aiger::import::AigerVar
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"AigerVar"}]

      Type alias used to indicate which variables use the AIGER numbering as opposed to the target
      numbering.
      
      Note that this is a type alias and thus does not provide any type checking.

   .. rubric:: Functions


   .. rust:function:: imctk_aiger::import::import_binary_aiger
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"import_binary_aiger"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"binary_aiger"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"io","target":"io"},{"type":"punctuation","value":"::"},{"type":"name","value":"Read"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Result","target":"Result"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerVar","target":"AigerVar"},{"type":"punctuation","value":", "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"link","value":"OrderedAig","target":"OrderedAig"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerLit","target":"AigerLit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"punctuation","value":", "},{"type":"link","value":"flussab_aiger","target":"flussab_aiger"},{"type":"punctuation","value":"::"},{"type":"name","value":"ParseError"},{"type":"punctuation","value":">"}]

      Read the passed data and parse it as binary AIGER file, importing it to the given target
      environment.

   .. rust:function:: imctk_aiger::import::import_ordered_aig
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"import_ordered_aig"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"aig"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"OrderedAig","target":"OrderedAig"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerLit","target":"AigerLit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerVar","target":"AigerVar"},{"type":"punctuation","value":", "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"}]

      Import an [`OrderedAig`] (representing a parsed binary AIGER file) to the given target
      environment.

   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_aiger::import::ExistingAigerVarMap
      :index: 1
      :vis: pub
      :toc: struct ExistingAigerVarMap
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"ExistingAigerVarMap"}]

      Partial mapping of AIGER variables to existing literals.

      .. rust:variable:: imctk_aiger::import::ExistingAigerVarMap::var_map
         :index: 2
         :vis: pub
         :toc: var_map
         :layout: [{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerVar","target":"AigerVar"},{"type":"punctuation","value":", "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

         Partial mapping of AIGER variables to existing literals.

      .. rust:variable:: imctk_aiger::import::ExistingAigerVarMap::latch_init
         :index: 2
         :vis: pub
         :toc: latch_init
         :layout: [{"type":"name","value":"latch_init"},{"type":"punctuation","value":": "},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"SteadyInput","target":"SteadyInput"},{"type":"punctuation","value":", "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

         Partial mapping of AIGER latch indices (represented as [`SteadyInput`]) to literals used for
         register initialization in the resulting internal represenation.

      .. rubric:: Implementations


      .. rust:impl:: imctk_aiger::import::ExistingAigerVarMap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ExistingAigerVarMap","target":"ExistingAigerVarMap"}]
         :toc: impl ExistingAigerVarMap


         .. rubric:: Functions


         .. rust:function:: imctk_aiger::import::ExistingAigerVarMap::import_binary_aiger
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"import_binary_aiger"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"punctuation","value":" + "},{"type":"link","value":"EnvWrapper","target":"EnvWrapper"},{"type":"punctuation","value":")"},{"type":"punctuation","value":", "},{"type":"name","value":"binary_aiger"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"io","target":"io"},{"type":"punctuation","value":"::"},{"type":"name","value":"Read"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Result","target":"Result"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerVar","target":"AigerVar"},{"type":"punctuation","value":", "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"link","value":"OrderedAig","target":"OrderedAig"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerLit","target":"AigerLit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"punctuation","value":", "},{"type":"link","value":"flussab_aiger","target":"flussab_aiger"},{"type":"punctuation","value":"::"},{"type":"name","value":"ParseError"},{"type":"punctuation","value":">"}]

            Read the passed data and parse it as binary AIGER file, importing it to the given target
            environment.

         .. rust:function:: imctk_aiger::import::ExistingAigerVarMap::import_ordered_aig
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"import_ordered_aig"},{"type":"punctuation","value":"("},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"punctuation","value":" + "},{"type":"link","value":"EnvWrapper","target":"EnvWrapper"},{"type":"punctuation","value":")"},{"type":"punctuation","value":", "},{"type":"name","value":"aig"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"OrderedAig","target":"OrderedAig"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerLit","target":"AigerLit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"AigerVar","target":"AigerVar"},{"type":"punctuation","value":", "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"}]

            Import an [`OrderedAig`] (representing a parsed binary AIGER file) to the given target
            environment.
