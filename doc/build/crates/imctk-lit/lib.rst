===================
Crate ``imctk_lit``
===================


.. rust:crate:: imctk_lit
   :index: 0

   Numeric identifiers for variables and Boolean literals

   .. rust:use:: imctk_lit
      :used_name: self


   .. rust:use:: imctk_lit::NegateInPlace
      :used_name: NegateInPlace


   .. rust:use:: imctk_lit::Negate
      :used_name: Negate


   .. rust:use:: imctk_lit::Var
      :used_name: Var


   .. rust:use:: imctk_lit::Lit
      :used_name: Lit


   .. rust:use:: imctk_lit::Pol
      :used_name: Pol


   .. rubric:: Traits


   .. rust:trait:: imctk_lit::Negate
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"Negate"}]

      Subtrait of `ops::Not` and `ops::BitXor<Pol>` for types that support Boolean negation.
      
      Types that implement `Negate` and `Clone` should also implement `Negate` for references.
      
      

      .. rubric:: Types


      .. rust:type:: imctk_lit::Negate::Negated
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"Negated"}]

         The common output type when invoking the `!` or `^` operator.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_lit::pol::&Lit::Negate
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Negate","target":"Negate"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'_"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl Negate for &Lit


      .. rust:impl:: imctk_lit::pol::u64::Negate
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Negate","target":"Negate"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]
         :toc: impl Negate for u64


      .. rust:impl:: imctk_lit::pol::&u64::Negate
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Negate","target":"Negate"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"u64","target":"u64"}]
         :toc: impl Negate for &u64


      .. rust:impl:: imctk_lit::pol::bool::Negate
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Negate","target":"Negate"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]
         :toc: impl Negate for bool


      .. rust:impl:: imctk_lit::pol::&bool::Negate
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Negate","target":"Negate"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"bool","target":"bool"}]
         :toc: impl Negate for &bool


      .. rust:impl:: imctk_lit::pol::&Pol::Negate
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Negate","target":"Negate"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl Negate for &Pol


   .. rust:trait:: imctk_lit::NegateInPlace
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"NegateInPlace"}]

      Subtrait of `Negate` and `ops::BitXorAssign<Pol>` for types that support in-place Boolean negation.
      
      

      .. rubric:: Functions


      .. rust:function:: imctk_lit::NegateInPlace::negate_in_place
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"negate_in_place"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

         Performs `ops::Not::not` in-place.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_lit::pol::u64::NegateInPlace
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NegateInPlace","target":"NegateInPlace"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]
         :toc: impl NegateInPlace for u64


      .. rust:impl:: imctk_lit::pol::bool::NegateInPlace
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NegateInPlace","target":"NegateInPlace"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]
         :toc: impl NegateInPlace for bool


   .. rubric:: Enums


   .. rust:enum:: imctk_lit::Pol
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"enum"},{"type":"space"},{"type":"name","value":"Pol"}]

      Either the identity function on, or negation of Booleans.
      
      With only two possible values, this type is itself isomorphic to `bool`, but using `bool`
      could be done by either representing the identity using `false` with `^` for application, or
      by representing the identity using `true` with `==` for application. Having a separte type
      to represent an invertible function on Booleans avoid having to make such an arbitrary
      choice, and makes the resulting code easier to read and makes it harder to introduce parity
      errors. In particular it should also prevent us from making different choices in different
      parts of the code base.
      
      

      .. rust:struct:: imctk_lit::Pol::Pos
         :index: 2
         :vis: pub
         :toc: Pos
         :layout: [{"type":"name","value":"Pos"}]

         Positive polarity, represents the identity function.

      .. rust:struct:: imctk_lit::Pol::Neg
         :index: 2
         :vis: pub
         :toc: Neg
         :layout: [{"type":"name","value":"Neg"}]

         Negative polarity, represents Boolean negation.

      .. rubric:: Implementations


      .. rust:impl:: imctk_lit::pol::Pol
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl Pol


         .. rubric:: Functions


         .. rust:function:: imctk_lit::pol::Pol::is_neg
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_neg"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` when this is the negative polarity.

         .. rust:function:: imctk_lit::pol::Pol::is_pos
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_pos"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` when this is the positive polarity.

         .. rust:function:: imctk_lit::pol::Pol::neg_if
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"neg_if"},{"type":"punctuation","value":"("},{"type":"name","value":"neg"},{"type":"punctuation","value":": "},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the negative polarity if the given condition is `true` and a positive polarity
            otherwise.

         .. rust:function:: imctk_lit::pol::Pol::pos_if
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"pos_if"},{"type":"punctuation","value":"("},{"type":"name","value":"pos"},{"type":"punctuation","value":": "},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the positive polarity if the given condition is `true` and a positive polarity
            otherwise.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_lit::pol::Pol::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl Debug for Pol


      .. rust:impl:: imctk_lit::pol::Pol::Display
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Display"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl Display for Pol


      .. rust:impl:: imctk_lit::pol::Pol::BitXor
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"BitXor"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl BitXor for Pol


      .. rust:impl:: imctk_lit::pol::Pol::BitXorAssign
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"BitXorAssign"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl BitXorAssign for Pol


      .. rust:impl:: imctk_lit::pol::Pol::Not
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"Not"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl Not for Pol


      .. rust:impl:: imctk_lit::pol::Pol::Negate
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Negate","target":"Negate"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl Negate for Pol


      .. rust:impl:: imctk_lit::pol::Pol::NegateInPlace
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NegateInPlace","target":"NegateInPlace"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]
         :toc: impl NegateInPlace for Pol


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_lit::Lit
      :index: 1
      :vis: pub
      :toc: struct Lit
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Lit"},{"type":"punctuation","value":"("},{"type":"link","value":"Id32","target":"Id32"},{"type":"punctuation","value":")"}]

      Numeric identifier for a Boolean-like literal.
      
      A literal consists of a [variable][`Var`] and a [polarity][`Pol`]. A literal with a positive
      polarity has the value of its variable while a literal with a negative polarity is obtained by
      applying the appropriate involution to the variable (usually some form of Boolean negation).
      
      The variable and polarity can be combined into a single number, called the
      [`code`][`Self::code`]. The variable index can be obtained by shifting the code one bit to the
      right with an even code corresponding to a positive and an odd code corresponding to a negative
      polarity literal.
      
      Note that the generic [`Id::id_index`] of a literal is the same as the [`code`][`Self::code`],
      not the [`index`][`Self::index`]. The index of a literal is the same as the corresponding
      variable's [`index`][`Var::index`].
      
      

      .. rubric:: Implementations


      .. rust:impl:: imctk_lit::lit::Lit
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl Lit


         .. rubric:: Variables


         .. rust:variable:: imctk_lit::lit::Lit::FALSE
            :index: -1
            :vis: pub
            :toc: const FALSE
            :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"FALSE"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"}]

            The literal representing constant false/0/low.

         .. rust:variable:: imctk_lit::lit::Lit::MAX_CODE
            :index: -1
            :vis: pub
            :toc: const MAX_CODE
            :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_CODE"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"}]

            The largest valid [`code`][`Self::code`] for a literal.

         .. rust:variable:: imctk_lit::lit::Lit::TRUE
            :index: -1
            :vis: pub
            :toc: const TRUE
            :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"TRUE"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"}]

            The literal representing constant true/1/high.

         .. rubric:: Functions


         .. rust:function:: imctk_lit::lit::Lit::as_neg
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"as_neg"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the negative polarity literal with the same variable.

         .. rust:function:: imctk_lit::lit::Lit::as_pos
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"as_pos"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the positive polarity literal with the same variable.

         .. rust:function:: imctk_lit::lit::Lit::code
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"code"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the literal's code.

         .. rust:function:: imctk_lit::lit::Lit::from_code
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_code"},{"type":"punctuation","value":"("},{"type":"name","value":"code"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the literal for a given code.

         .. rust:function:: imctk_lit::lit::Lit::from_code_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_code_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"code"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the literal for a given code without bounds checking.
            # Safety
            The caller needs to ensure that `code <= Lit::MAX_CODE`.

         .. rust:function:: imctk_lit::lit::Lit::index
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"index"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the index of the literal's variable.

         .. rust:function:: imctk_lit::lit::Lit::is_const
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_const"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether this literal is [`Self::FALSE`] or [`Self::TRUE`].

         .. rust:function:: imctk_lit::lit::Lit::is_neg
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_neg"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether this literal has negative polarity.

         .. rust:function:: imctk_lit::lit::Lit::is_pos
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_pos"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether this literal has positive polarity.

         .. rust:function:: imctk_lit::lit::Lit::lookup
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"lookup"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Negate","target":"Negate"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnOnce","target":"FnOnce"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Negated"}]

            This is equivalent to `f(self.var()) ^ self.pol()`.

         .. rust:function:: imctk_lit::lit::Lit::pol
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"pol"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Pol","target":"Pol"}]

            Returns the polarity of the literal.

         .. rust:function:: imctk_lit::lit::Lit::var
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"var"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]

            Returns the variable of the literal.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_lit::lit::Lit::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl Debug for Lit


      .. rust:impl:: imctk_lit::lit::Lit::Display
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Display"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl Display for Lit


      .. rust:impl:: imctk_lit::lit::Lit::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl Default for Lit


      .. rust:impl:: imctk_lit::lit::Lit::From
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"From","target":"From"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl From for Lit


      .. rust:impl:: imctk_lit::lit::Lit::BitXor
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"BitXor"},{"type":"punctuation","value":"<"},{"type":"link","value":"Pol","target":"Pol"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl BitXor for Lit


      .. rust:impl:: imctk_lit::lit::Lit::BitXorAssign
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"BitXorAssign"},{"type":"punctuation","value":"<"},{"type":"link","value":"Pol","target":"Pol"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl BitXorAssign for Lit


      .. rust:impl:: imctk_lit::lit::Lit::BitXor
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"BitXor"},{"type":"punctuation","value":"<"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl BitXor for Lit


      .. rust:impl:: imctk_lit::lit::Lit::BitXorAssign
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"BitXorAssign"},{"type":"punctuation","value":"<"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl BitXorAssign for Lit


      .. rust:impl:: imctk_lit::lit::Lit::Not
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"Not"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl Not for Lit


      .. rust:impl:: imctk_lit::lit::Lit::Negate
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Negate","target":"Negate"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl Negate for Lit


      .. rust:impl:: imctk_lit::lit::Lit::NegateInPlace
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"NegateInPlace","target":"NegateInPlace"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl NegateInPlace for Lit


      .. rust:impl:: imctk_lit::lit::Lit::Lit
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"flussab_aiger","target":"flussab_aiger"},{"type":"punctuation","value":"::"},{"type":"name","value":"Lit"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]
         :toc: impl Lit for Lit


   .. rust:struct:: imctk_lit::Var
      :index: 1
      :vis: pub
      :toc: struct Var
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Var"},{"type":"punctuation","value":"("},{"type":"link","value":"GenericId","target":"GenericId"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"<const arg>"},{"type":"punctuation","value":", "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":"::"},{"type":"name","value":"BaseId"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

      Numeric identifier for a variable.
      
      

      .. rubric:: Implementations


      .. rust:impl:: imctk_lit::var::Var
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]
         :toc: impl Var


         .. rubric:: Variables


         .. rust:variable:: imctk_lit::var::Var::FALSE
            :index: -1
            :vis: pub
            :toc: const FALSE
            :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"FALSE"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"}]

            The variable representing constant false/0/low.

         .. rust:variable:: imctk_lit::var::Var::MAX_INDEX
            :index: -1
            :vis: pub
            :toc: const MAX_INDEX
            :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"}]

            The largest valid [`index`][`Self::index`] for a variable.

         .. rubric:: Functions


         .. rust:function:: imctk_lit::var::Var::as_lit
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"as_lit"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

            Returns a positive polarity literal for the variable.

         .. rust:function:: imctk_lit::var::Var::as_neg_lit
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"as_neg_lit"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

            Returns a negative polarity literal for the variable.

         .. rust:function:: imctk_lit::var::Var::from_index
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_index"},{"type":"punctuation","value":"("},{"type":"name","value":"index"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the variable for a given index.

         .. rust:function:: imctk_lit::var::Var::from_index_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_index_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"index"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the variable for a given index without bounds checking.
            # Safety
            The caller needs to ensure that `index <= Var::MAX_INDEX`.

         .. rust:function:: imctk_lit::var::Var::index
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"index"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the index of the variable.

         .. rust:function:: imctk_lit::var::Var::lit
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"lit"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"pol"},{"type":"punctuation","value":": "},{"type":"link","value":"Pol","target":"Pol"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

            Returns the literal for this variable with the given polarity.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_lit::var::Var::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]
         :toc: impl Debug for Var


      .. rust:impl:: imctk_lit::var::Var::Display
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Display"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]
         :toc: impl Display for Var


      .. rust:impl:: imctk_lit::var::Var::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]
         :toc: impl Default for Var


      .. rust:impl:: imctk_lit::var::Var::BitXor
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"BitXor"},{"type":"punctuation","value":"<"},{"type":"link","value":"Pol","target":"Pol"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]
         :toc: impl BitXor for Var


      .. rust:impl:: imctk_lit::var::Var::BitXor
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"BitXor"},{"type":"punctuation","value":"<"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]
         :toc: impl BitXor for Var

