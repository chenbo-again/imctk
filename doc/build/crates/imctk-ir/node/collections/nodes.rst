=============
``mod nodes``
=============


.. rust:module:: imctk_ir::node::collections::nodes
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ir::node::collections::nodes
      :used_name: self


   .. rust:use:: std::hash::BuildHasher
      :used_name: BuildHasher


   .. rust:use:: std::ptr::null_mut
      :used_name: null_mut


   .. rust:use:: std::fmt
      :used_name: fmt


   .. rust:use:: std::mem::ManuallyDrop
      :used_name: ManuallyDrop


   .. rust:use:: std::hash::BuildHasherDefault
      :used_name: BuildHasherDefault


   .. rust:use:: std::alloc::Layout
      :used_name: Layout


   .. rust:use:: std::fmt::Debug
      :used_name: Debug


   .. rust:use:: hashbrown::HashTable
      :used_name: HashTable


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: imctk_ids::indexed_id_vec::IndexedIdVec
      :used_name: IndexedIdVec


   .. rust:use:: imctk_util::give_take::Take
      :used_name: Take


   .. rust:use:: zwohash::ZwoHasher
      :used_name: ZwoHasher


   .. rust:use:: imctk_ir::node::generic::Node
      :used_name: Node


   .. rust:use:: imctk_ir::node::generic::DynNode
      :used_name: DynNode


   .. rust:use:: imctk_ir::node::generic::NodeType
      :used_name: NodeType


   .. rust:use:: imctk_ir::node::vtables::KnownNodeType
      :used_name: KnownNodeType


   .. rust:use:: imctk_ir::node::NodeId
      :used_name: NodeId


   .. rust:use:: imctk_ir::node::vtables::GenericNodeType
      :used_name: GenericNodeType


   .. rust:use:: imctk_ir::node::vtables::DynNodeType
      :used_name: DynNodeType


   .. rubric:: Enums


   .. rust:enum:: imctk_ir::node::collections::nodes::NodeError
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"enum"},{"type":"space"},{"type":"name","value":"NodeError"}]

      Error cases for accesses a [`Node`] among [`Nodes`] using a given [`NodeId`] and [`NodeType`].

      .. rust:struct:: imctk_ir::node::collections::nodes::NodeError::NotPresent
         :index: 2
         :vis: pub
         :toc: NotPresent
         :layout: [{"type":"name","value":"NotPresent"}]

         The [`NodeId`] is not present in the collection of [`Nodes`].

      .. rust:struct:: imctk_ir::node::collections::nodes::NodeError::UnexpectedNodeType
         :index: 2
         :vis: pub
         :toc: UnexpectedNodeType
         :layout: [{"type":"name","value":"UnexpectedNodeType"}]

         The [`NodeId`] is present but does not have the given [`NodeType`].

         .. rust:variable:: imctk_ir::node::collections::nodes::NodeError::UnexpectedNodeType::found_type
            :index: -1
            :vis: pub
            :toc: found_type
            :layout: [{"type":"name","value":"found_type"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeType","target":"NodeType"}]

            The actual type of the found node.

   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ir::node::collections::nodes::Nodes
      :index: 1
      :vis: pub
      :toc: struct Nodes
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Nodes"}]

      A heterogeneous collection of [`Node`] values.
      
      Every inserted node is assigned a [`NodeId`] which can be used to look up, modify or delete
      nodes. The order in which node ids are assigned and/or re-used is deterministic for the same
      sequence of operations, but otherwise unspecified and may change across versions.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::node::collections::nodes::Nodes
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Nodes","target":"Nodes"}]
         :toc: impl Nodes


         .. rubric:: Functions


         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::discard
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"discard"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Removes the [`Node`] with a given [`NodeId`].
            
            Returns `false` when there was no node of the given id and `true` when such a node was
            removed.
            
            This drops the node in-place, see [`remove`][Self::remove] for a method that returns the
            removed node, but requires a statically known node type.

         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::get_dyn
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_dyn"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":">"}]

            Returns a trait object reference to the node with a given [`NodeId`].
            
            Returns `None` if the collection has no node of the given id.

         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::get_dyn_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_dyn_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":">"}]

            Returns a mutable trait object reference to the node with a given [`NodeId`].
            
            Returns `None` if the collection has no node of the given id.

         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Node","target":"Node"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"}]

            Insert a [`Node`] with a statically known node type.

         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` when the collection of nodes is empty.

         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"punctuation","value":"("},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"}]

            Iterate over all nodes, yielding the assigned id paired with a trait object reference for
            each node.
            
            The results are ordered by id.

         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of contained nodes.

         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::node_type_stats
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"node_type_stats"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"punctuation","value":"("},{"type":"link","value":"NodeType","target":"NodeType"},{"type":"punctuation","value":", "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Iterate over the used node types, yielding each corresponding [`NodeType`] paired with the
            number of contained nodes of that type.

         .. rust:function:: imctk_ir::node::collections::nodes::Nodes::remove_dyn_with
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove_dyn_with"},{"type":"punctuation","value":"<"},{"type":"name","value":"R"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"FnOnce","target":"FnOnce"},{"type":"punctuation","value":"("},{"type":"link","value":"Take","target":"Take"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"R","target":"R"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"R","target":"R"},{"type":"punctuation","value":">"}]

            Removes the [`Node`] with a given [`NodeId`], passing an ownership-transferring dynamically
            typed reference to a callback closure.
            
            When a node with the given id was found, this will return the callback result and `None`
            otherwise.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::node::collections::nodes::Nodes::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Nodes","target":"Nodes"}]
         :toc: impl Debug for Nodes

