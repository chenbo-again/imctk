===============
``mod circuit``
===============


.. rust:module:: imctk_ir::node::fine::circuit
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ir::node::fine::circuit
      :used_name: self


   .. rust:use:: imctk_ids::Id32
      :used_name: Id32


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: imctk_util::unordered_pair::UnorderedPair
      :used_name: UnorderedPair


   .. rust:use:: imctk_ir::node::generic::default_reduce_node
      :used_name: default_reduce_node


   .. rust:use:: imctk_ir::node::generic::TermNode
      :used_name: TermNode


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rust:use:: imctk_ir::node::generic::SealedWrapper
      :used_name: SealedWrapper


   .. rust:use:: imctk_ir::node::generic::TermDyn
      :used_name: TermDyn


   .. rust:use:: imctk_ir::var::Pol
      :used_name: Pol


   .. rust:use:: imctk_ir::node::generic::Term
      :used_name: Term


   .. rust:use:: imctk_ir::node::builder::NodeBuilder
      :used_name: NodeBuilder


   .. rust:use:: imctk_ir::node::generic::Node
      :used_name: Node


   .. rust:use:: super::constraints::BinClause
      :used_name: BinClause


   .. rubric:: Types


   .. rust:type:: imctk_ir::node::fine::circuit::AndNode
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"AndNode"}]

      [`Node`] representing the Boolean 'and' of two values.

   .. rust:type:: imctk_ir::node::fine::circuit::InitNode
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"InitNode"}]

      [`Node`] representing the initial value of any given variable.

   .. rust:type:: imctk_ir::node::fine::circuit::InputNode
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"InputNode"}]

      [`Node`] representing a time-varying input or an unconstrained time-varying value.

   .. rust:type:: imctk_ir::node::fine::circuit::RegNode
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"RegNode"}]

      [`Node`] representing a register that updates with each transition of the represented state
      transition system.

   .. rust:type:: imctk_ir::node::fine::circuit::SteadyInputNode
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"SteadyInputNode"}]

      [`Node`] representing a steady input or a steady unconstrained constant value.

   .. rust:type:: imctk_ir::node::fine::circuit::XorNode
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"XorNode"}]

      [`Node`] representing the Boolean 'xor' ('exclusive or').

   .. rubric:: Traits


   .. rust:trait:: imctk_ir::node::fine::circuit::FineCircuitNodeBuilder
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"FineCircuitNodeBuilder"}]

      Extension trait to more conveniently add [fine-grained circuit terms][`self`] to any
      [`NodeBuilder`].

      .. rubric:: Functions


      .. rust:function:: imctk_ir::node::fine::circuit::FineCircuitNodeBuilder::and
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"and"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"inputs"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Into","target":"Into"},{"type":"punctuation","value":"<"},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

         Adds an [`And`] term to the environment.

      .. rust:function:: imctk_ir::node::fine::circuit::FineCircuitNodeBuilder::init
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"init"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"input"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

         Adds a [`Init`] term to the environment, automatically normalizing polarities as required.

      .. rust:function:: imctk_ir::node::fine::circuit::FineCircuitNodeBuilder::or
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"or"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"inputs"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Into","target":"Into"},{"type":"punctuation","value":"<"},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

         Adds an [`And`] term to the environment that computes the Boolean 'or' of two inputs using
         De Morgan's laws.

      .. rust:function:: imctk_ir::node::fine::circuit::FineCircuitNodeBuilder::reg
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"reg"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"init"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":", "},{"type":"name","value":"next"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

         Adds a [`Reg`] term to the environment, automatically normalizing polarities as required.

      .. rust:function:: imctk_ir::node::fine::circuit::FineCircuitNodeBuilder::xor
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"xor"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"inputs"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Into","target":"Into"},{"type":"punctuation","value":"<"},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

         Adds a [`Xor`] term to the environment, automatically normalizing polarities as required.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::node::fine::circuit::T::FineCircuitNodeBuilder
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeBuilder","target":"NodeBuilder"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"FineCircuitNodeBuilder","target":"FineCircuitNodeBuilder"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl FineCircuitNodeBuilder for T


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ir::node::fine::circuit::And
      :index: 1
      :vis: pub
      :toc: struct And
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"And"}]

      [`Term`] representing the Boolean 'and' of two values.
      
      This is a combinational operation.

      .. rust:variable:: imctk_ir::node::fine::circuit::And::inputs
         :index: 2
         :vis: pub
         :toc: inputs
         :layout: [{"type":"name","value":"inputs"},{"type":"punctuation","value":": "},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"}]

         The operands for the 'and' given as input literals.

   .. rust:struct:: imctk_ir::node::fine::circuit::Init
      :index: 1
      :vis: pub
      :toc: struct Init
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Init"}]

      [`Term`] representing the initial value of any given variable.

      .. rust:variable:: imctk_ir::node::fine::circuit::Init::input
         :index: 2
         :vis: pub
         :toc: input
         :layout: [{"type":"name","value":"input"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"}]

         The variable this term represents the initial value of.

   .. rust:struct:: imctk_ir::node::fine::circuit::Input
      :index: 1
      :vis: pub
      :toc: struct Input
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Input"},{"type":"punctuation","value":"("},{"type":"link","value":"Id32","target":"Id32"},{"type":"punctuation","value":")"}]

      [`Term`] representing a time-varying input or unconstrained time-varying value.

   .. rust:struct:: imctk_ir::node::fine::circuit::Reg
      :index: 1
      :vis: pub
      :toc: struct Reg
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Reg"}]

      [`Term`] representing a register that updates with each transition of the represented state transition system.
      
      In the initial state it transparently passes through the `init` input, and after every
      transition it will output the value the `next` input had before the transition.
      
      When viewing time-varying signals as infinite streams this can also be seen as the "cons"
      operation on infinite streams.

      .. rust:variable:: imctk_ir::node::fine::circuit::Reg::init
         :index: 2
         :vis: pub
         :toc: init
         :layout: [{"type":"name","value":"init"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"}]

         Produced value in the initial state.

      .. rust:variable:: imctk_ir::node::fine::circuit::Reg::next
         :index: 2
         :vis: pub
         :toc: next
         :layout: [{"type":"name","value":"next"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"}]

         Value to produce after the next transition.

   .. rust:struct:: imctk_ir::node::fine::circuit::SteadyInput
      :index: 1
      :vis: pub
      :toc: struct SteadyInput
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"SteadyInput"},{"type":"punctuation","value":"("},{"type":"link","value":"Id32","target":"Id32"},{"type":"punctuation","value":")"}]

      [`Term`] representing a steady input or unconstrained steady value.
      
      An important use case is unconstrained register initialization, where this provides the initial
      value.

   .. rust:struct:: imctk_ir::node::fine::circuit::Xor
      :index: 1
      :vis: pub
      :toc: struct Xor
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Xor"}]

      [`Term`] representing Boolean 'xor' ('exclusive or').
      
      This is a combinational operation.

      .. rust:variable:: imctk_ir::node::fine::circuit::Xor::inputs
         :index: 2
         :vis: pub
         :toc: inputs
         :layout: [{"type":"name","value":"inputs"},{"type":"punctuation","value":": "},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"}]

         The operands for the 'xor' given as input literals.
