================
``mod glucose2``
================


.. rust:module:: imctk_abc::sat::glucose2
   :index: 0
   :vis: pub


   .. rust:use:: imctk_abc::sat::glucose2
      :used_name: self


   .. rust:use:: std::marker::PhantomData
      :used_name: PhantomData


   .. rust:use:: std::mem::take
      :used_name: take


   .. rust:use:: std::ffi::c_int
      :used_name: c_int


   .. rust:use:: std::cell::RefCell
      :used_name: RefCell


   .. rust:use:: std::os::raw::c_void
      :used_name: c_void


   .. rust:use:: std::mem::ManuallyDrop
      :used_name: ManuallyDrop


   .. rust:use:: std::collections::HashSet
      :used_name: HashSet


   .. rust:use:: imctk_abc_sys
      :used_name: abc


   .. rust:use:: imctk_ir::var::Var
      :used_name: Var


   .. rust:use:: imctk_ir::var::Lit
      :used_name: Lit


   .. rust:use:: imctk_util::unordered_pair::UnorderedPair
      :used_name: UnorderedPair


   .. rust:use:: imctk_abc::sat::glucose2::sealed::SolverMode
      :used_name: SolverMode


   .. rust:use:: imctk_abc::sat::glucose2::sealed::CircuitMode
      :used_name: CircuitMode


   .. rubric:: Traits


   .. rust:trait:: imctk_abc::sat::glucose2::ProofTracer
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"ProofTracer"}]

      Trait to receive proof-tracing callbacks while solving.

      .. rubric:: Functions


      .. rust:function:: imctk_abc::sat::glucose2::ProofTracer::conflict
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"conflict"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"conflict_lits"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":", "},{"type":"name","value":"tags"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"u32","target":"u32"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":", "},{"type":"name","value":"units"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"}]

         Callback invoked when the assumptions are detected to be in conflict.

      .. rust:function:: imctk_abc::sat::glucose2::ProofTracer::learnt_clause
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"learnt_clause"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"clause_lits"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":", "},{"type":"name","value":"tags"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"u32","target":"u32"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":", "},{"type":"name","value":"units"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u32","target":"u32"}]

         Callback invoked when learning a clause.

      .. rust:function:: imctk_abc::sat::glucose2::ProofTracer::learnt_unit
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"learnt_unit"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"unit"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":", "},{"type":"name","value":"tag"},{"type":"punctuation","value":": "},{"type":"link","value":"u32","target":"u32"},{"type":"punctuation","value":", "},{"type":"name","value":"units"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"}]

         Callback invoked when learning a unit clause.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_abc::sat::glucose2::&RefCell::ProofTracer
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"ProofTracer","target":"ProofTracer"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ProofTracer","target":"ProofTracer"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'_"},{"type":"space"},{"type":"link","value":"RefCell","target":"RefCell"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl ProofTracer for &RefCell


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_abc::sat::glucose2::CircuitJust
      :index: 1
      :vis: pub
      :toc: struct CircuitJust
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"CircuitJust"}]

      CNF only propagation and circuit based justification

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_abc::sat::glucose2::CircuitJust::SolverMode
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"sealed","target":"sealed"},{"type":"punctuation","value":"::"},{"type":"name","value":"SolverMode"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"CircuitJust","target":"CircuitJust"}]
         :toc: impl SolverMode for CircuitJust


      .. rust:impl:: imctk_abc::sat::glucose2::CircuitJust::CircuitMode
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"sealed","target":"sealed"},{"type":"punctuation","value":"::"},{"type":"name","value":"CircuitMode"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"CircuitJust","target":"CircuitJust"}]
         :toc: impl CircuitMode for CircuitJust


   .. rust:struct:: imctk_abc::sat::glucose2::CircuitProp
      :index: 1
      :vis: pub
      :toc: struct CircuitProp
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"CircuitProp"}]

      CNF and circuit propagation with circuit based justification

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_abc::sat::glucose2::CircuitProp::SolverMode
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"sealed","target":"sealed"},{"type":"punctuation","value":"::"},{"type":"name","value":"SolverMode"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"CircuitProp","target":"CircuitProp"}]
         :toc: impl SolverMode for CircuitProp


      .. rust:impl:: imctk_abc::sat::glucose2::CircuitProp::CircuitMode
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"sealed","target":"sealed"},{"type":"punctuation","value":"::"},{"type":"name","value":"CircuitMode"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"CircuitProp","target":"CircuitProp"}]
         :toc: impl CircuitMode for CircuitProp


   .. rust:struct:: imctk_abc::sat::glucose2::CnfOnly
      :index: 1
      :vis: pub
      :toc: struct CnfOnly
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"CnfOnly"}]

      CNF only propagation and no justification

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_abc::sat::glucose2::CnfOnly::SolverMode
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"sealed","target":"sealed"},{"type":"punctuation","value":"::"},{"type":"name","value":"SolverMode"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"CnfOnly","target":"CnfOnly"}]
         :toc: impl SolverMode for CnfOnly


   .. rust:struct:: imctk_abc::sat::glucose2::Solver
      :index: 1
      :vis: pub
      :toc: struct Solver
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Solver"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"Mode"},{"type":"punctuation","value":": "},{"type":"link","value":"SolverMode","target":"SolverMode"},{"type":"punctuation","value":">"}]

      An instance of abc's glucose2 SAT solver

      .. rubric:: Implementations


      .. rust:impl:: imctk_abc::sat::glucose2::Solver
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"Mode"},{"type":"punctuation","value":": "},{"type":"link","value":"SolverMode","target":"SolverMode"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Solver","target":"Solver"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"Mode","target":"Mode"},{"type":"punctuation","value":">"}]
         :toc: impl Solver


         .. rubric:: Functions


         .. rust:function:: imctk_abc::sat::glucose2::Solver::add_and
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"add_and"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"output"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"name","value":"inputs"},{"type":"punctuation","value":": "},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Adds an AND gate to the solver.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::add_clause
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"add_clause"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"lits"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"}]

            Adds a CNF clause to the solver.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::add_tagged_clause
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"add_tagged_clause"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"lits"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":", "},{"type":"name","value":"tag"},{"type":"punctuation","value":": "},{"type":"link","value":"u32","target":"u32"},{"type":"punctuation","value":")"}]

            Adds a CNF clause with a given proof tracing tag.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::add_xor
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"add_xor"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"output"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"name","value":"inputs"},{"type":"punctuation","value":": "},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Adds a XOR gate to the solver.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::clear_limits
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear_limits"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Clears all configured limits.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::conflicts
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"conflicts"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]

            Returns the number of conflicts that occured in total.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::cumulative_conflict_limit
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"cumulative_conflict_limit"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"limit"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Sets a conflict limit for future solves.
            
            This limit remains in place for multiple solves, with each solve reducing the
            limit by the number of conflicts that occured.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::failed_assumptions
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"failed_assumptions"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"}]

            Returns a subset of assumptions that made the last solve unsatisfiable.
            
            Returns an emtpy slice when the last call was not unsatisfiable.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::mark_cone
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"mark_cone"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"}]

            Extend the circuit based justification propagation window by the input cone of a variable.
            
            The inputs are only traversed up to any node that is already part of the propagation window.
            Hence, when [`Self::mark_var`] was called before, this might not add the full transitive
            input cone to the propagation window.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::mark_var
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"mark_var"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"}]

            Extend the circuit based justification propagation window by a single variable.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::new_round
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"new_round"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Reset the propagation window for circuit based justification.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::produce_inner_model
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"produce_inner_model"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"produce_inner"},{"type":"punctuation","value":": "},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"}]

            When using circuit based justification, include inner circuit nodes in the produced model.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::reset
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"reset"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Reset the solver back to the initial state
            
            Equivalent to creating a new solver, but avoids newly allocating solver resources.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::solve_assuming
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"solve_assuming"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"assumptions"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":">"}]

            Find a solution justifying the given assumptions.
            
            Returns `Some(true)` when a solution was fond, `Some(false)` when no solution exists and
            `None` when reaching any configured limit.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::try_input_model
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"try_input_model"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":">"}]

            Returns the input model justifying the assumptions when in circuit based justification mode.
            
            When the last solve wasn't satisfiable or when not in circuit based justification mode, this
            will return `None`.
            
            When `Self::produce_inner_model` was used to request an inner model in addition to the
            inputs, this will contain both the inputs and inner nodes needed to justify the assumptions.

      .. rust:impl:: imctk_abc::sat::glucose2::Solver
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"Mode"},{"type":"punctuation","value":": "},{"type":"link","value":"SolverMode","target":"SolverMode"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Solver","target":"Solver"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"Mode","target":"Mode"},{"type":"punctuation","value":">"}]
         :toc: impl Solver


         .. rubric:: Functions


         .. rust:function:: imctk_abc::sat::glucose2::Solver::stop_proof_tracing
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"stop_proof_tracing"},{"type":"punctuation","value":"("},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Solver","target":"Solver"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'static"},{"type":"punctuation","value":", "},{"type":"link","value":"Mode","target":"Mode"},{"type":"punctuation","value":">"}]

            Detach any currently attached proof tracing callback.

         .. rust:function:: imctk_abc::sat::glucose2::Solver::with_proof_tracer
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"with_proof_tracer"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"ProofTracer","target":"ProofTracer"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"tracer"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Solver","target":"Solver"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"Mode","target":"Mode"},{"type":"punctuation","value":">"}]

            Attach a proof tracing callback.

      .. rust:impl:: imctk_abc::sat::glucose2::Solver
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"Mode"},{"type":"punctuation","value":": "},{"type":"link","value":"CircuitMode","target":"CircuitMode"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Solver","target":"Solver"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"Mode","target":"Mode"},{"type":"punctuation","value":">"}]
         :toc: impl Solver


         .. rubric:: Functions


         .. rust:function:: imctk_abc::sat::glucose2::Solver::input_model
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"input_model"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":"]"}]

            Returns the input model justifying the assumptions.
            
            This panics when the last solve wasn't satisfiable.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_abc::sat::glucose2::Solver::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"Mode"},{"type":"punctuation","value":": "},{"type":"link","value":"SolverMode","target":"SolverMode"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Solver","target":"Solver"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'static"},{"type":"punctuation","value":", "},{"type":"link","value":"Mode","target":"Mode"},{"type":"punctuation","value":">"}]
         :toc: impl Default for Solver


      .. rust:impl:: imctk_abc::sat::glucose2::Solver::Drop
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"Mode"},{"type":"punctuation","value":": "},{"type":"link","value":"SolverMode","target":"SolverMode"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Drop","target":"Drop"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Solver","target":"Solver"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"Mode","target":"Mode"},{"type":"punctuation","value":">"}]
         :toc: impl Drop for Solver

