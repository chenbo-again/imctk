===========
``mod buf``
===========


.. rust:module:: imctk_ir::node::collections::buf
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ir::node::collections::buf
      :used_name: self


   .. rust:use:: std::fmt::Debug
      :used_name: Debug


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: imctk_util::give_take::Take
      :used_name: Take


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_ir::node::generic::TermWrapper
      :used_name: TermWrapper


   .. rust:use:: imctk_ir::node::NodeId
      :used_name: NodeId


   .. rust:use:: imctk_ir::node::generic::DynNode
      :used_name: DynNode


   .. rust:use:: imctk_ir::node::generic::dyn_term_into_dyn_term_wrapper
      :used_name: dyn_term_into_dyn_term_wrapper


   .. rust:use:: imctk_ir::node::vtables::DynTermType
      :used_name: DynTermType


   .. rust:use:: imctk_ir::node::vtables::GenericTermType
      :used_name: GenericTermType


   .. rust:use:: imctk_ir::node::vtables::DynNodeType
      :used_name: DynNodeType


   .. rust:use:: imctk_ir::node::generic::dyn_term_wrapper_as_dyn_term
      :used_name: dyn_term_wrapper_as_dyn_term


   .. rust:use:: imctk_ir::node::vtables::GenericNodeType
      :used_name: GenericNodeType


   .. rust:use:: imctk_ir::node::generic::Term
      :used_name: Term


   .. rust:use:: imctk_ir::node::generic::DynTerm
      :used_name: DynTerm


   .. rust:use:: imctk_ir::node::generic::Node
      :used_name: Node


   .. rust:use:: imctk_ir::node::builder::NodeBuilderDyn
      :used_name: NodeBuilderDyn


   .. rust:use:: imctk_ir::node::builder::NodeBuilder
      :used_name: NodeBuilder


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ir::node::collections::buf::NodeBuf
      :index: 1
      :vis: pub
      :toc: struct NodeBuf
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"NodeBuf"}]

      A mostly write-only collection of nodes, terms and equivalences stored outside of an
      environment.
      
      While individual nodes, terms and equivalences can be added using the [`NodeBuilder`] trait, the
      only way to access the added items is by using [`Self::drain_into_node_builder`] to add all of
      them to another [`NodeBuilder`] (e.g. an environment that does provide access to individual
      nodes).
      
      Note that when adding terms, a [`NodeBuf`] will allocate fresh variable with ids decreasing from
      the maximal supported variable id. These will get remapped to fresh or existing equivalent
      variables when draining the contents into another [`NodeBuilder`]. The resulting mapping is
      returned as a [`NodeBufVarMap`] reference.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::node::collections::buf::NodeBuf
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuf","target":"NodeBuf"}]
         :toc: impl NodeBuf


         .. rubric:: Functions


         .. rust:function:: imctk_ir::node::collections::buf::NodeBuf::drain_into_node_builder
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"drain_into_node_builder"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"builder"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"NodeBufVarMap","target":"NodeBufVarMap"}]

            Adds the contents of this buffer into another [`NodeBuilder`] and clear this buffer.
            
            This returns a [`NodeBufVarMap`] reference that contains the mapping for any variables
            freshly allocated for [`Term`] outputs.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::node::collections::buf::NodeBuf::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"NodeBuf","target":"NodeBuf"}]
         :toc: impl Debug for NodeBuf


      .. rust:impl:: imctk_ir::node::collections::buf::NodeBuf::NodeBuilderDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilderDyn","target":"NodeBuilderDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"NodeBuf","target":"NodeBuf"}]
         :toc: impl NodeBuilderDyn for NodeBuf


      .. rust:impl:: imctk_ir::node::collections::buf::NodeBuf::NodeBuilder
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"NodeBuf","target":"NodeBuf"}]
         :toc: impl NodeBuilder for NodeBuf


   .. rust:struct:: imctk_ir::node::collections::buf::NodeBufVarMap
      :index: 1
      :vis: pub
      :toc: struct NodeBufVarMap
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"NodeBufVarMap"}]

      Variable mapping created when draining the contents of a [`NodeBuf`].

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::node::collections::buf::NodeBufVarMap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBufVarMap","target":"NodeBufVarMap"}]
         :toc: impl NodeBufVarMap


         .. rubric:: Functions


         .. rust:function:: imctk_ir::node::collections::buf::NodeBufVarMap::map_var
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"map_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

            Applies the mapping determined when draining the contents of a [`NodeBuf`] into a
            [`NodeBuilder`]. This is an identity mapping for any variable or literal that was explicitly
            added to the [`NodeBuf`] but will remap the freshly allocated variables for [`Term`]
            outputs.
