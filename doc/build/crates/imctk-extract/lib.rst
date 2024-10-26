=======================
Crate ``imctk_extract``
=======================


.. rust:crate:: imctk_extract
   :index: 0

   Utilities for extracting parts of an environment.

   .. rust:use:: imctk_extract
      :used_name: self


   .. rust:use:: std::collections::BTreeSet
      :used_name: BTreeSet


   .. rust:use:: imctk_ids::Id32
      :used_name: Id32


   .. rust:use:: imctk_ids::id_vec::IdVec
      :used_name: IdVec


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_ir::node::NodeId
      :used_name: NodeId


   .. rust:use:: imctk_ir::env::VarDef
      :used_name: VarDef


   .. rust:use:: imctk_ir::env::Env
      :used_name: Env


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rust:use:: imctk_util::vec_sink::VecSink
      :used_name: VecSink


   .. rust:use:: imctk_util::topo_sorted_sccs::TopoSortedSccs
      :used_name: TopoSortedSccs


   .. rubric:: Functions


   .. rust:function:: imctk_extract::duplicate
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"duplicate"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

      Returns a copy of the environment with topologically sorted variable ids.

   .. rust:function:: imctk_extract::duplicate_partial
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"duplicate_partial"},{"type":"punctuation","value":"("},{"type":"name","value":"zelf"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"targets"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

      Returns a copy of the environment limited to the input cones of the given targets, using
      topologically sorted variable ids.

   .. rust:function:: imctk_extract::extract_topo_sorted_primary_defs
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"extract_topo_sorted_primary_defs"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"targets"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"}]

      Extract topologically sorted transitive input cones in the primary definition graph.
      
      This starts with a set of target variables and computes their combined transitive input cone.
      The input cone computation will follow guarded inputs that are allowed to form primary
      definition cycles (e.g. the "next" input of registers).
      
      For topologocailly sorting the nodes of the input cone guarded inputs are ignored. If the
      resulting input cones contain an unguarded cyle, this method will panic.

   .. rust:function:: imctk_extract::partial_extract_topo_sorted_primary_defs
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"partial_extract_topo_sorted_primary_defs"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"targets"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"expand"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"}]

      Extract topologically sorted partial transitive input cones in the primary definition graph.

   .. rust:function:: imctk_extract::primary_def_unguarded_input_vars
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"primary_def_unguarded_input_vars"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

      Predecessor callback for [`TopoSortedSccs`] that traverses unguarded inputs of an environment's
      primary definition graph.

   .. rust:function:: imctk_extract::select_primary_defs
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"select_primary_defs"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":")"}]

      Select new primary definitions for all variables in the environment.

   .. rust:function:: imctk_extract::select_primary_defs_by
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"select_primary_defs_by"},{"type":"punctuation","value":"("},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"candidate"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"}]

      Select new primary definitions for all variables in the environment using a user provided
      predicate to select preferred candidates.
