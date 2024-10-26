===============
``mod extract``
===============


.. rust:module:: imctk_aiger::extract
   :index: 0
   :vis: pub


   .. rust:use:: imctk_aiger::extract
      :used_name: self


   .. rust:use:: flussab_aiger::aig::OrderedLatch
      :used_name: OrderedLatch


   .. rust:use:: flussab_aiger::aig::OrderedAndGate
      :used_name: OrderedAndGate


   .. rust:use:: flussab_aiger::aig::OrderedAig
      :used_name: OrderedAig


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: imctk_ids::id_vec::IdVec
      :used_name: IdVec


   .. rust:use:: imctk_ir::node::fine::circuit::InputNode
      :used_name: InputNode


   .. rust:use:: imctk_ir::node::fine::circuit::AndNode
      :used_name: AndNode


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rust:use:: imctk_ir::node::fine::circuit::SteadyInputNode
      :used_name: SteadyInputNode


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_ir::env::Env
      :used_name: Env


   .. rust:use:: imctk_ir::node::fine::circuit::InitNode
      :used_name: InitNode


   .. rust:use:: imctk_ir::node::fine::circuit::XorNode
      :used_name: XorNode


   .. rust:use:: imctk_ir::node::fine::circuit::RegNode
      :used_name: RegNode


   .. rust:use:: imctk_util::vec_sink::VecSink
      :used_name: VecSink


   .. rust:use:: zwohash::HashMap
      :used_name: HashMap


   .. rubric:: Functions


   .. rust:function:: imctk_aiger::extract::extract_aiger
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"extract_aiger"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"outputs"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"ExtractedAig","target":"ExtractedAig"}]

      Extracts an AIGER compatible sequential AIG for the given sequence of output literals.
      
      This will tranparently expand imctk Xor terms into an equivalent network of 3 AIG And gates.

   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_aiger::extract::ExtractedAig
      :index: 1
      :vis: pub
      :toc: struct ExtractedAig
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"ExtractedAig"}]

      An extracted AIG together with extraction metadata.

      .. rust:variable:: imctk_aiger::extract::ExtractedAig::aig
         :index: 2
         :vis: pub
         :toc: aig
         :layout: [{"type":"name","value":"aig"},{"type":"punctuation","value":": "},{"type":"link","value":"flussab_aiger","target":"flussab_aiger"},{"type":"punctuation","value":"::"},{"type":"name","value":"aig"},{"type":"punctuation","value":"::"},{"type":"name","value":"OrderedAig"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"}]

         The extracted AIG

      .. rust:variable:: imctk_aiger::extract::ExtractedAig::aig_from_env
         :index: 2
         :vis: pub
         :toc: aig_from_env
         :layout: [{"type":"name","value":"aig_from_env"},{"type":"punctuation","value":": "},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

         Maps environment variables to equivalent AIG literals

      .. rust:variable:: imctk_aiger::extract::ExtractedAig::init_repr_for_steady
         :index: 2
         :vis: pub
         :toc: init_repr_for_steady
         :layout: [{"type":"name","value":"init_repr_for_steady"},{"type":"punctuation","value":": "},{"type":"link","value":"HashMap","target":"HashMap"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"}]

         Maps enviornment variables to environment literals that are equivalent in the initial time
         step and are used to represent the initial value in the extracted AIG
