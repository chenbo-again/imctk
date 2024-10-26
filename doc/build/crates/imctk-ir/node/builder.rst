===============
``mod builder``
===============


.. rust:module:: imctk_ir::node::builder
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ir::node::builder
      :used_name: self


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_util::give_take::Take
      :used_name: Take


   .. rust:use:: imctk_util::give_take::give
      :used_name: give


   .. rust:use:: super::generic::Node
      :used_name: Node


   .. rust:use:: super::generic::DynTerm
      :used_name: DynTerm


   .. rust:use:: super::generic::Term
      :used_name: Term


   .. rust:use:: super::generic::DynNode
      :used_name: DynNode


   .. rubric:: Types


   .. rust:type:: imctk_ir::node::builder::DynNodeBuilder
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"DynNodeBuilder"}]

      A trait object for a [`NodeBuilder`].
      
      Since [`NodeBuilder`] is not object safe, we cannot use `dyn NodeBuilder`. Instead have to use a
      separate [`NodeBuilderDyn`] trait and provide this type alias to avoid the need for `dyn
      NodeBuilderDyn`.

   .. rubric:: Traits


   .. rust:trait:: imctk_ir::node::builder::NodeBuilder
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"NodeBuilder"}]

      Types with support for adding equivalences and nodes, including term nodes with automatically
      assigned output variables.
      
      Everything that is object-safe is part of the [`NodeBuilderDyn`] supertrait.

      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::builder::NodeBuilder::node
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"node"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Node","target":"Node"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"}]

         Ensure the presence of the given node.

      .. rust:function:: imctk_ir::node::builder::NodeBuilder::term
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"term"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Term","target":"Term"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"term"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Output"}]

         Ensure the presence of a term node for the given term, returning the assigned output
         variable or literal.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::builder::DynNodeBuilder::NodeBuilder
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"DynNodeBuilder","target":"DynNodeBuilder"}]
         :toc: impl NodeBuilder for DynNodeBuilder


      .. rust:impl:: imctk_ir::node::builder::Env::NodeBuilder
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Env","target":"Env"}]
         :toc: impl NodeBuilder for Env


      .. rust:impl:: imctk_ir::node::builder::DefBuilder::NodeBuilder
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"DefBuilder","target":"DefBuilder"}]
         :toc: impl NodeBuilder for DefBuilder


   .. rust:trait:: imctk_ir::node::builder::NodeBuilderDyn
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"NodeBuilderDyn"}]

      Object-safe supertrait for [`NodeBuilder`].

      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::builder::NodeBuilderDyn::dyn_node
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_node"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"link","value":"Take","target":"Take"},{"type":"punctuation","value":"<"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         Ensure the presence of the given dynamically typed node.

      .. rust:function:: imctk_ir::node::builder::NodeBuilderDyn::dyn_term
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dyn_term"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"term"},{"type":"punctuation","value":": "},{"type":"link","value":"Take","target":"Take"},{"type":"punctuation","value":"<"},{"type":"link","value":"DynTerm","target":"DynTerm"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

         Ensure the presence of a dynamically typed term node, returning the assigned output
         literal.
         
         If the term node has a non-Boolean output, the polarity of the returned literal will be
         positive.

      .. rust:function:: imctk_ir::node::builder::NodeBuilderDyn::equiv
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"equiv"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"equiv"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"; "},{"type":"literal","value":"2"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"}]

         Add an equivalence of literals.

      .. rust:function:: imctk_ir::node::builder::NodeBuilderDyn::valid_temporary_vars
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"valid_temporary_vars"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"count"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

         Returns whether the largest `count` variables are unused by the builder and thus safe to use
         as temporary variables.
         
         Note that it's usually not a good idea to add any nodes, terms or equivalences containing
         such variables to the builder, as this will allocate all preceding variables.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::builder::Env::NodeBuilderDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilderDyn","target":"NodeBuilderDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Env","target":"Env"}]
         :toc: impl NodeBuilderDyn for Env


      .. rust:impl:: imctk_ir::node::builder::DefBuilder::NodeBuilderDyn
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NodeBuilderDyn","target":"NodeBuilderDyn"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"DefBuilder","target":"DefBuilder"}]
         :toc: impl NodeBuilderDyn for DefBuilder

