=============
``mod index``
=============


.. rust:module:: imctk_ir::index
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ir::index
      :used_name: self


   .. rust:use:: std::hash::BuildHasher
      :used_name: BuildHasher


   .. rust:use:: std::hash::BuildHasherDefault
      :used_name: BuildHasherDefault


   .. rust:use:: table_seq::TableSeq
      :used_name: TableSeq


   .. rust:use:: zwohash::ZwoHasher
      :used_name: ZwoHasher


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rust:use:: imctk_ir::node::generic::DynNode
      :used_name: DynNode


   .. rust:use:: imctk_ir::node::generic::Node
      :used_name: Node


   .. rust:use:: imctk_ir::node::collections::nodes::Nodes
      :used_name: Nodes


   .. rust:use:: imctk_ir::node::NodeId
      :used_name: NodeId


   .. rust:use:: super::node::generic::TermNode
      :used_name: TermNode


   .. rust:use:: super::node::generic::DynTerm
      :used_name: DynTerm


   .. rust:use:: super::var::Lit
      :used_name: Lit


   .. rust:use:: super::env::NodeRole
      :used_name: NodeRole


   .. rust:use:: super::node::generic::Term
      :used_name: Term


   .. rubric:: Traits


   .. rust:trait:: imctk_ir::index::DynamicIndex
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"DynamicIndex"}]

      Types that maintain a dynamic index of an environment.

      .. rubric:: Types


      .. rust:type:: imctk_ir::index::DynamicIndex::Context
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"Context"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":">"}]

         Additional context required for index maintenance.

      .. rubric:: Functions


      .. rust:function:: imctk_ir::index::DynamicIndex::add_dyn_node
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"add_dyn_node"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"context"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Context"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":", "},{"type":"name","value":"node_role"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeRole","target":"NodeRole"},{"type":"punctuation","value":")"}]

         Process the addition of a new dynamically typed node.

      .. rust:function:: imctk_ir::index::DynamicIndex::add_equiv
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"add_equiv"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"context"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Context"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"name","value":"repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":", "},{"type":"name","value":"equiv"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"}]

         Process the addition of a literal equivalence.

      .. rust:function:: imctk_ir::index::DynamicIndex::add_node
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"add_node"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Node","target":"Node"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"context"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Context"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"node_role"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeRole","target":"NodeRole"},{"type":"punctuation","value":")"}]

         Process the addition of a new statically typed node.
         
         By default, this forwards to the dynamically typed version and indices cannot expect
         additions and removals to be consistent w.r.t. static vs. dynamic typing.

      .. rust:function:: imctk_ir::index::DynamicIndex::change_primary_def
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"change_primary_def"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"context"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Context"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"name","value":"old_primary_def"},{"type":"punctuation","value":": "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"name","value":"new_primary_def"},{"type":"punctuation","value":": "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         Process a change in the node considered the primary definition for a variable.

      .. rust:function:: imctk_ir::index::DynamicIndex::remove_dyn_node
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove_dyn_node"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"context"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Context"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":", "},{"type":"name","value":"node_role"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeRole","target":"NodeRole"},{"type":"punctuation","value":")"}]

         Process the removal of a new dynamically typed node.

      .. rust:function:: imctk_ir::index::DynamicIndex::remove_node
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove_node"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Node","target":"Node"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"context"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Context"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"node_role"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeRole","target":"NodeRole"},{"type":"punctuation","value":")"}]

         Process the removal of a new statically typed node.
         
         By default, this forwards to the dynamically typed version and indices cannot expect
         additions and removals to be consistent w.r.t. static vs. dynamic typing.

   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ir::index::DefsIndex
      :index: 1
      :vis: pub
      :toc: struct DefsIndex
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"DefsIndex"}]

      Index to look up non-primary definition nodes that have a given variable as output.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::index::DefsIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"DefsIndex","target":"DefsIndex"}]
         :toc: impl DefsIndex


         .. rubric:: Functions


         .. rust:function:: imctk_ir::index::DefsIndex::find_non_primary_defs_unordered
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"find_non_primary_defs_unordered"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Returns the node ids of all nodes that use the given variable as output and are not the
            primary definition for that variable.
            
            The order of the resulting iterator is unspecified and should not be dependent upon for any
            oreder-sensitive operation.

         .. rust:function:: imctk_ir::index::DefsIndex::non_primary_def_count
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"non_primary_def_count"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of nodes that use the given variable as output and are not the primary
            definition for that variable.
            
            This is a constant time operation.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::index::DefsIndex::DynamicIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"DynamicIndex","target":"DynamicIndex"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"DefsIndex","target":"DefsIndex"}]
         :toc: impl DynamicIndex for DefsIndex


   .. rust:struct:: imctk_ir::index::FoundNode
      :index: 1
      :vis: pub
      :toc: struct FoundNode
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"FoundNode"}]

      A node found by the [`StructuralHashIndex`].

      .. rust:variable:: imctk_ir::index::FoundNode::node_id
         :index: 2
         :vis: pub
         :toc: node_id
         :layout: [{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"}]

         The id of the found node.

      .. rust:variable:: imctk_ir::index::FoundNode::equiv
         :index: 2
         :vis: pub
         :toc: equiv
         :layout: [{"type":"name","value":"equiv"},{"type":"punctuation","value":": "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"; "},{"type":"literal","value":"2"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":">"}]

         The output equivalence implied by the target and found node.

   .. rust:struct:: imctk_ir::index::StructuralHashIndex
      :index: 1
      :vis: pub
      :toc: struct StructuralHashIndex
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"StructuralHashIndex"}]

      Index to look up a [`NodeId`] given a defining [`Node`] or [`Term`].

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::index::StructuralHashIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"StructuralHashIndex","target":"StructuralHashIndex"}]
         :toc: impl StructuralHashIndex


         .. rubric:: Functions


         .. rust:function:: imctk_ir::index::StructuralHashIndex::find_dyn_node
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"find_dyn_node"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"nodes"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Nodes","target":"Nodes"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"FoundNode","target":"FoundNode"},{"type":"punctuation","value":">"}]

            Find an existing [`Node`] that may differ from the given node in the used output
            variable/literal. Dynamically typed version.

         .. rust:function:: imctk_ir::index::StructuralHashIndex::find_dyn_term
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"find_dyn_term"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"nodes"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Nodes","target":"Nodes"},{"type":"punctuation","value":", "},{"type":"name","value":"term"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"DynTerm","target":"DynTerm"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"}]

            Find an existing [`TermNode`] for a given dynamically typed [`Term`].

         .. rust:function:: imctk_ir::index::StructuralHashIndex::find_node
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"find_node"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Node","target":"Node"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"nodes"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Nodes","target":"Nodes"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"FoundNode","target":"FoundNode"},{"type":"punctuation","value":">"}]

            Find an existing [`Node`] that may differ from the given node in the used output
            variable/literal. Statically typed version.

         .. rust:function:: imctk_ir::index::StructuralHashIndex::find_term
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"find_term"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Term","target":"Term"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"nodes"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Nodes","target":"Nodes"},{"type":"punctuation","value":", "},{"type":"name","value":"term"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Output"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"}]

            Find an existing [`TermNode`] for a given [`Term`].

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::index::StructuralHashIndex::DynamicIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"DynamicIndex","target":"DynamicIndex"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"StructuralHashIndex","target":"StructuralHashIndex"}]
         :toc: impl DynamicIndex for StructuralHashIndex


   .. rust:struct:: imctk_ir::index::UsesIndex
      :index: 1
      :vis: pub
      :toc: struct UsesIndex
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"UsesIndex"}]

      Index to look up nodes given one of their input variables.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::index::UsesIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"UsesIndex","target":"UsesIndex"}]
         :toc: impl UsesIndex


         .. rubric:: Functions


         .. rust:function:: imctk_ir::index::UsesIndex::find_uses_unordered
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"find_uses_unordered"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Returns the node ids of all nodes that use the given variable as input.
            
            The order of the resulting iterator is unspecified and should not be dependent upon for any
            oreder-sensitive operation.

         .. rust:function:: imctk_ir::index::UsesIndex::use_count
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"use_count"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of nodes that use the given variable as input.
            
            This is a constant time operation.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::index::UsesIndex::DynamicIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"DynamicIndex","target":"DynamicIndex"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"UsesIndex","target":"UsesIndex"}]
         :toc: impl DynamicIndex for UsesIndex

