===================
``mod constraints``
===================


.. rust:module:: imctk_ir::node::fine::constraints
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ir::node::fine::constraints
      :used_name: self


   .. rust:use:: imctk_util::unordered_pair::UnorderedPair
      :used_name: UnorderedPair


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rust:use:: imctk_ir::node::builder::NodeBuilder
      :used_name: NodeBuilder


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_ir::node::generic::SealedWrapper
      :used_name: SealedWrapper


   .. rust:use:: imctk_ir::node::generic::Node
      :used_name: Node


   .. rust:use:: imctk_ir::node::generic::NodeDyn
      :used_name: NodeDyn


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ir::node::fine::constraints::BinClause
      :index: 1
      :vis: pub
      :toc: struct BinClause
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"BinClause"}]

      A binary clause, requiring at least one of two inputs to be true at any time.

      .. rust:variable:: imctk_ir::node::fine::constraints::BinClause::inputs
         :index: 2
         :vis: pub
         :toc: inputs
         :layout: [{"type":"name","value":"inputs"},{"type":"punctuation","value":": "},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"}]

         The inputs for the binary clause constraint.
