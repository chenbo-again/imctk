===============
``mod generic``
===============


.. rust:module:: imctk_ir::node::generic
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ir::node::generic
      :used_name: self


   .. rust:use:: std::hash::BuildHasherDefault
      :used_name: BuildHasherDefault


   .. rust:use:: std::hash::BuildHasher
      :used_name: BuildHasher


   .. rust:use:: std::hash::Hash
      :used_name: Hash


   .. rust:use:: std::fmt::Debug
      :used_name: Debug


   .. rust:use:: imctk_transparent::SubtypeCast
      :used_name: SubtypeCast


   .. rust:use:: imctk_transparent::NewtypeCast
      :used_name: NewtypeCast


   .. rust:use:: imctk_util::vec_sink::VecSink
      :used_name: VecSink


   .. rust:use:: imctk_util::give_take::Take
      :used_name: Take


   .. rust:use:: zwohash::ZwoHasher
      :used_name: ZwoHasher


   .. rust:use:: imctk_ir::var::Pol
      :used_name: Pol


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_ir::env::Env
      :used_name: Env


   .. rust:use:: super::vtables::GenericNodeType
      :used_name: GenericNodeType


   .. rust:use:: super::vtables::DynNodeType
      :used_name: DynNodeType


   .. rust:use:: super::collections::buf::NodeBuf
      :used_name: NodeBuf


   .. rust:use:: super::vtables::DynTermType
      :used_name: DynTermType


   .. rust:use:: super::builder::NodeBuilder
      :used_name: NodeBuilder


   .. rust:use:: super::vtables::GenericTermType
      :used_name: GenericTermType


   .. rust:use:: imctk_ir::node::generic::sealed::SealedWrapper
      :used_name: SealedWrapper
      :reexport: imctk_ir::node::generic


   .. rust:use:: super::vtables::NodeType
      :used_name: NodeType
      :reexport: imctk_ir::node::generic


   .. rust:use:: super::vtables::TermType
      :used_name: TermType
      :reexport: imctk_ir::node::generic

   .. rubric:: Re-exports

   * :rust:any:`super::vtables::NodeType`
   * :rust:any:`super::vtables::TermType`

   .. rubric:: Types


   .. rust:type:: imctk_ir::node::generic::DynNode
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"DynNode"}]

      A trait object for a [`Node`].
      
      Since [`Node`] is not object safe, we cannot use `dyn Node`. Instead have to use a separate
      [`NodeDyn`] trait and provide this type alias to avoid the need for `dyn NodeDyn`.

   .. rust:type:: imctk_ir::node::generic::DynTerm
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"DynTerm"}]

      A trait object for a [`Term`].
      
      Since [`Term`] is not object safe, we cannot use `dyn Term`. Instead have to use a separate
      [`TermDyn`] trait and provide this type alias to avoid the need for `dyn TermDyn`.

   .. rubric:: Functions


   .. rust:function:: imctk_ir::node::generic::default_reduce_node
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"default_reduce_node"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Term","target":"Term"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"name","value":"term"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"output"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Output"},{"type":"punctuation","value":", "},{"type":"name","value":"builder"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

      Default implementation of [`Term::reduce_node`].
      
      This default implementation performs reduction using [`Term::reduce`], adding an equivalence of
      the returned new output and the output previously present in the [`TermNode`].
      
      This should only be called when implementing [`Term::reduce_node`] to partially override the
      default behavior.

   .. rubric:: Traits


   .. rust:trait:: imctk_ir::node::generic::Node
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"Node"}]

      Allows using a type for internal represenation nodes.
      
      Everything that is object-safe is part of the [`NodeDyn`] supertrait.

      .. rubric:: Variables


      .. rust:variable:: imctk_ir::node::generic::Node::NAME
         :index: -1
         :vis: pub
         :toc: const NAME
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"NAME"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'static"},{"type":"space"},{"type":"link","value":"str","target":"str"}]

         A short name identifying the node type.

      .. rust:variable:: imctk_ir::node::generic::Node::STATIC_TYPE_INFO
         :index: -1
         :vis: pub
         :toc: const STATIC_TYPE_INFO
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"STATIC_TYPE_INFO"},{"type":"punctuation","value":": "},{"type":"link","value":"SealedWrapper","target":"SealedWrapper"},{"type":"punctuation","value":"<"},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"}]


      .. rust:variable:: imctk_ir::node::generic::Node::TERM_TYPE_FOR_TERM_WRAPPER
         :index: -1
         :vis: pub
         :toc: const TERM_TYPE_FOR_TERM_WRAPPER
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"TERM_TYPE_FOR_TERM_WRAPPER"},{"type":"punctuation","value":": "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"SealedWrapper","target":"SealedWrapper"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"TermType","target":"TermType"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]


      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::generic::Node::apply_var_map
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"apply_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"}]

         Rewrites all variables in the term using a given mapping.

      .. rust:function:: imctk_ir::node::generic::Node::def_eq
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"def_eq"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"other"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Returns whether two nodes can be treated as equivalent.
         
         In particular for [`TermNode`] nodes, the output variable or literal is ignored as it is
         fully defined in terms of the input variables.
         
         This defaults to forwarding to [`Eq`], which is correct for constraint nodes.

      .. rust:function:: imctk_ir::node::generic::Node::input_var_iter
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"input_var_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

         Returns an iterator over all input variables of the node.

      .. rust:function:: imctk_ir::node::generic::Node::reduce
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"reduce"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"builder"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Performs node-local simplifying rewrites.
         
         This can either update the node in-place, returning `false` or produce replacement nodes in
         the passed [`NodeBuilder`], returning `true`.
         
         It is an error to produce nodes in the builder when returning `false`.

      .. rust:function:: imctk_ir::node::generic::Node::unguarded_input_var_iter
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"unguarded_input_var_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

         Returns an iterator over all input variables that should be taken into consideration when
         maintaining acyclicity.
         
         The default implementation forwards to [`input_var_iter`][Self::input_var_iter].

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::generic::BinClause::Node
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Node","target":"Node"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"BinClause","target":"BinClause"}]
         :toc: impl Node for BinClause


   .. rust:trait:: imctk_ir::node::generic::NodeDyn
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"NodeDyn"}]

      Object-safe supertrait for [`Node`].

      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::generic::NodeDyn::def_hash
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"def_hash"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]

         Returns a hash value of the defining part of a node.
         
         In particular for [`TermNode`] nodes, the output variable or literal is ignored as it is
         fully defined in terms of the input variables.
         
         This defaults to forwarding to [`Hash`] using a [`ZwoHasher`].

      .. rust:function:: imctk_ir::node::generic::NodeDyn::dyn_term
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_term"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"DynTerm","target":"DynTerm"},{"type":"punctuation","value":">"}]

         Returns the term whose value this node assigns to the node's output or `None` if this node
         is not a [`TermNode`].

      .. rust:function:: imctk_ir::node::generic::NodeDyn::max_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"max_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]

         Returns the variable with the largest id among all variables referenced by this node.

      .. rust:function:: imctk_ir::node::generic::NodeDyn::output_lit
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"output_lit"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"}]

         Returns the output literal defined by this node or `None` if this node does not have an
         output.
         
         If the output is a variable, this should return a positive polarity literal for that
         variable, which is what the default implementation does.

      .. rust:function:: imctk_ir::node::generic::NodeDyn::output_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"output_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"}]

         Returns the output variable defined by this node or `None` if this node does not have an
         output.
         
         If the output is a literal, the literal's variable is still considered an output variable in
         the context of this method.

      .. rust:function:: imctk_ir::node::generic::NodeDyn::representative_input_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"representative_input_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]

         Returns a representative input variable.
         
         This is used like a hash value that is restricted to the contained variables or
         `Var::FALSE` and enables certain internal optimizations.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::generic::BinClause::NodeDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeDyn","target":"NodeDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"BinClause","target":"BinClause"}]
         :toc: impl NodeDyn for BinClause


   .. rust:trait:: imctk_ir::node::generic::NodeDynAuto
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"NodeDynAuto"}]

      Automatically implemented object-safe supertrait for [`Node`].
      
      This contains object-safe methods that will be automatically implemented via a blanket
      implementation using the provided [`Node`] and [`NodeDyn`] items.

      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_add_to_buf_with_var_map
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_add_to_buf_with_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"buf"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"NodeBuf","target":"NodeBuf"},{"type":"punctuation","value":", "},{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"}]

         Adds a copy of this node to the given node buffer using a given variable mapping.

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_add_to_env_with_var_map
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_add_to_env_with_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"}]

         Adds a copy of this node to the given environment using a given variable mapping.

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_append_input_vars
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_append_input_vars"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"sink"},{"type":"punctuation","value":": "},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         Alternative object safe wrapper of [`Node::input_var_iter`].
         
         See [`Self::dyn_foreach_input_var`] for a version that uses a dynamic callback instead of a
         [`VecSink`].

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_append_unguarded_input_vars
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_append_unguarded_input_vars"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"sink"},{"type":"punctuation","value":": "},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         Alternative object safe wrapper of [`Node::unguarded_input_var_iter`].
         
         See [`Self::dyn_foreach_unguarded_input_var`] for a version that uses a dynamic callback
         instead of a [`VecSink`].

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_apply_var_map
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_apply_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var_repr"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"}]

         Object safe wrapper of [`Node::apply_var_map`].

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_def_eq
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_def_eq"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"other"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Object safe wrapper of [`Node::def_eq`].

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_foreach_input_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_foreach_input_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"}]

         Object safe wrapper of [`Node::input_var_iter`].
         
         See [`Self::dyn_foreach_input_var`] for a version that uses a
         [`VecSink`] instead of a dynamic callback.

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_foreach_unguarded_input_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_foreach_unguarded_input_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"}]

         Object safe wrapper of [`Node::unguarded_input_var_iter`].
         
         See [`Self::dyn_append_unguarded_input_vars`] for a version that uses a [`VecSink`] instead
         of a dynamic callback.

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::dyn_reduce_into_buf
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_reduce_into_buf"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"buf"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"NodeBuf","target":"NodeBuf"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Object safe wrapper of [`Node::reduce`].

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::node_type
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"node_type"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"NodeType","target":"NodeType"}]

         Returns the dynamic [`NodeType`] corresponding to the concrete [`Node`] implementation
         for trait object.

      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::zzz_hidden_default_def_hash
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"zzz_hidden_default_def_hash"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]


      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::zzz_hidden_default_max_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"zzz_hidden_default_max_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]


      .. rust:function:: imctk_ir::node::generic::NodeDynAuto::zzz_hidden_default_representative_input_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"zzz_hidden_default_representative_input_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]


      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::generic::T::NodeDynAuto
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Node","target":"Node"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NodeDynAuto","target":"NodeDynAuto"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl NodeDynAuto for T


   .. rust:trait:: imctk_ir::node::generic::Term
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"Term"}]

      Types that define a value within an environment.
      
      A [`Term`] type defines a value in an [environment][crate::env]. That value can be given as
      a function of values assigned to [variables][crate::var] in the environment.
      
      A term itself is not automatically assigned to a variable. This is done by combining a term and
      output variable (or literal) in a [`TermNode`].
      
      Everything that is object-safe is part of the [`TermDyn`] supertrait.

      .. rubric:: Types


      .. rust:type:: imctk_ir::node::generic::Term::Output
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"Output"}]

         Whether the output can be represented as a variable or can require a literal.

      .. rubric:: Variables


      .. rust:variable:: imctk_ir::node::generic::Term::NAME
         :index: -1
         :vis: pub
         :toc: const NAME
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"NAME"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'static"},{"type":"space"},{"type":"link","value":"str","target":"str"}]

         A short name identifying the operation.

      .. rust:variable:: imctk_ir::node::generic::Term::STATIC_TYPE_INFO
         :index: -1
         :vis: pub
         :toc: const STATIC_TYPE_INFO
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"STATIC_TYPE_INFO"},{"type":"punctuation","value":": "},{"type":"link","value":"SealedWrapper","target":"SealedWrapper"},{"type":"punctuation","value":"<"},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"}]


      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::generic::Term::apply_var_map
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"apply_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]

         Rewrites all variables in the term using a given mapping.

      .. rust:function:: imctk_ir::node::generic::Term::def_eq
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"def_eq"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"other"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Returns whether two [`Term`]s define the same value.
         
         This defaults to forwarding to [`Eq`].

      .. rust:function:: imctk_ir::node::generic::Term::input_var_iter
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"input_var_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

         Returns an iterator over all input variables of the term.

      .. rust:function:: imctk_ir::node::generic::Term::is_steady
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_steady"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"input_steady"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Returns whether this term represents a steady value given a callback to determine whether
         the inputs represent steady values.

      .. rust:function:: imctk_ir::node::generic::Term::reduce
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"reduce"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"buf"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Output"},{"type":"punctuation","value":">"}]

         Performs term-local simplifying rewrites.
         
         This can either update the term in-place, returning `None` or produce replacement nodes in
         the passed [`NodeBuilder`], returning an output variable or literal equivalent to the value
         of this term.
         
         It is an error to produce nodes in the builder when returning `None`.

      .. rust:function:: imctk_ir::node::generic::Term::reduce_node
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"reduce_node"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"output"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Output"},{"type":"punctuation","value":", "},{"type":"name","value":"builder"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Performs node-local simplifying rewrites of a term node holding this term.
         
         This is invoked when calling [`Node::reduce`] on a [`TermNode`].
         
         The default implementation performs reduction using [`Self::reduce`], adding an equivalence
         of the returned new output and the output previously present in the [`TermNode`].
         
         The default implementation is accessible as [`default_reduce_node`], which allows
         implementations to partially override the default behavior.

      .. rust:function:: imctk_ir::node::generic::Term::unguarded_input_var_iter
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"unguarded_input_var_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

         Returns an iterator over all input variables that should be taken into consideration when
         maintaining acyclicity.
         
         The default implementation forwards to [`input_var_iter`][Self::input_var_iter].

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::generic::And::Term
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Term","target":"Term"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"And","target":"And"}]
         :toc: impl Term for And


      .. rust:impl:: imctk_ir::node::generic::Xor::Term
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Term","target":"Term"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Xor","target":"Xor"}]
         :toc: impl Term for Xor


      .. rust:impl:: imctk_ir::node::generic::SteadyInput::Term
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Term","target":"Term"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SteadyInput","target":"SteadyInput"}]
         :toc: impl Term for SteadyInput


      .. rust:impl:: imctk_ir::node::generic::Input::Term
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Term","target":"Term"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Input","target":"Input"}]
         :toc: impl Term for Input


      .. rust:impl:: imctk_ir::node::generic::Reg::Term
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Term","target":"Term"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Reg","target":"Reg"}]
         :toc: impl Term for Reg


      .. rust:impl:: imctk_ir::node::generic::Init::Term
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Term","target":"Term"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Init","target":"Init"}]
         :toc: impl Term for Init


   .. rust:trait:: imctk_ir::node::generic::TermDyn
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"TermDyn"}]

      Object-safe supertrait for [`Term`].

      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::generic::TermDyn::def_hash
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"def_hash"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]

         Returns a hash value for the value defined by this term.
         
         This defaults to forwarding to [`Hash`] using a [`ZwoHasher`].

      .. rust:function:: imctk_ir::node::generic::TermDyn::max_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"max_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]

         Returns the variable with the largest id among all variables referenced by this term.

      .. rust:function:: imctk_ir::node::generic::TermDyn::representative_input_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"representative_input_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]

         Returns a representative input variable.
         
         This is used like a hash value that is restricted to the contained variables or
         `Var::FALSE` and enables certain internal optimizations.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::generic::And::TermDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"TermDyn","target":"TermDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"And","target":"And"}]
         :toc: impl TermDyn for And


      .. rust:impl:: imctk_ir::node::generic::Xor::TermDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"TermDyn","target":"TermDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Xor","target":"Xor"}]
         :toc: impl TermDyn for Xor


      .. rust:impl:: imctk_ir::node::generic::SteadyInput::TermDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"TermDyn","target":"TermDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SteadyInput","target":"SteadyInput"}]
         :toc: impl TermDyn for SteadyInput


      .. rust:impl:: imctk_ir::node::generic::Input::TermDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"TermDyn","target":"TermDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Input","target":"Input"}]
         :toc: impl TermDyn for Input


      .. rust:impl:: imctk_ir::node::generic::Reg::TermDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"TermDyn","target":"TermDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Reg","target":"Reg"}]
         :toc: impl TermDyn for Reg


      .. rust:impl:: imctk_ir::node::generic::Init::TermDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"TermDyn","target":"TermDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Init","target":"Init"}]
         :toc: impl TermDyn for Init


   .. rust:trait:: imctk_ir::node::generic::TermDynAuto
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"TermDynAuto"}]

      Automatically implemented object-safe supertrait for [`Term`].
      
      This contains object-safe methods that will be automatically implemented via a blanket
      implementation using the provided [`Term`] and [`TermDyn`] items.

      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_add_to_buf_with_var_map
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_add_to_buf_with_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"buf"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"NodeBuf","target":"NodeBuf"},{"type":"punctuation","value":", "},{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

         Adds a copy of this term to the given node buffer using a given variable mapping.

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_add_to_env_with_var_map
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_add_to_env_with_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

         Adds a copy of this term to the given environment using a given variable mapping.

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_append_input_vars
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_append_input_vars"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"sink"},{"type":"punctuation","value":": "},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         Alternative object safe wrapper of [`Term::input_var_iter`].
         
         See [`Self::dyn_foreach_input_var`] for a version that uses a dynamic callback instead of a
         [`VecSink`].

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_append_unguarded_input_vars
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_append_unguarded_input_vars"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"sink"},{"type":"punctuation","value":": "},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         Alternative object safe wrapper of [`Term::unguarded_input_var_iter`].
         
         See [`Self::dyn_foreach_unguarded_input_var`] for a version that uses a dynamic callback
         instead of a [`VecSink`].

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_apply_var_map
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_apply_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var_repr"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]

         Object safe wrapper of [`Term::apply_var_map`].

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_def_eq
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_def_eq"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"other"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"DynTerm","target":"DynTerm"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Object safe wrapper of [`Term::def_eq`].

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_foreach_input_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_foreach_input_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"}]

         Object safe wrapper of [`Term::input_var_iter`].
         
         See [`Self::dyn_foreach_input_var`] for a version that uses a [`VecSink`] instead of a
         dynamic callback.

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_foreach_unguarded_input_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_foreach_unguarded_input_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"dyn"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"}]

         Object safe wrapper of [`Term::unguarded_input_var_iter`].
         
         See [`Self::dyn_append_unguarded_input_vars`] for a version that uses a [`VecSink`] instead
         of a dynamic callback.

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_is_steady_in_env
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_is_steady_in_env"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Object safe wrapper of [`Term::is_steady`] using an environment to look up input steadiness.

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::dyn_reduce_into_buf
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_reduce_into_buf"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"buf"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"NodeBuf","target":"NodeBuf"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"}]

         Object safe wrapper of [`Term::reduce`].

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::term_type
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"term_type"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"TermType","target":"TermType"}]

         Returns the dynamic [`TermType`] corresponding to the concrete [`Term`] implementation
         for trait object.

      .. rust:function:: imctk_ir::node::generic::TermDynAuto::zzz_hidden_default_def_hash
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"zzz_hidden_default_def_hash"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]


      .. rust:function:: imctk_ir::node::generic::TermDynAuto::zzz_hidden_default_max_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"zzz_hidden_default_max_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]


      .. rust:function:: imctk_ir::node::generic::TermDynAuto::zzz_hidden_default_representative_input_var
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"zzz_hidden_default_representative_input_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]


      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::generic::T::TermDynAuto
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Term","target":"Term"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"TermDynAuto","target":"TermDynAuto"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl TermDynAuto for T


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ir::node::generic::TermNode
      :index: 1
      :vis: pub
      :toc: struct TermNode
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"TermNode"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Term","target":"Term"},{"type":"punctuation","value":">"}]

      Node type to assign a variable for the value defined by a [`Term`].

      .. rust:variable:: imctk_ir::node::generic::TermNode::output
         :index: 2
         :vis: pub
         :toc: output
         :layout: [{"type":"name","value":"output"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Output"}]

         The variable or literal holding the value defined by the term.

      .. rust:variable:: imctk_ir::node::generic::TermNode::term
         :index: 2
         :vis: pub
         :toc: term
         :layout: [{"type":"name","value":"term"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"}]

         The term defining the value of the output variable.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::node::generic::TermNode::Node
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Term","target":"Term"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Node","target":"Node"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"TermNode","target":"TermNode"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Node for TermNode


      .. rust:impl:: imctk_ir::node::generic::TermNode::NodeDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Term","target":"Term"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NodeDyn","target":"NodeDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"TermNode","target":"TermNode"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl NodeDyn for TermNode

