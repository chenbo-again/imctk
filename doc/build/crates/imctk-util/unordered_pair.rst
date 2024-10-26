======================
``mod unordered_pair``
======================


.. rust:module:: imctk_util::unordered_pair
   :index: 0
   :vis: pub


   .. rust:use:: imctk_util::unordered_pair
      :used_name: self


   .. rust:use:: std::ops
      :used_name: ops


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_util::unordered_pair::UnorderedPair
      :index: 1
      :vis: pub
      :toc: struct UnorderedPair
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      An unordered pair represented as a sorted `[T; 2]`.
      
      Note that this does allow a pair containing the same value twice.

      .. rubric:: Implementations


      .. rust:impl:: imctk_util::unordered_pair::UnorderedPair
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Ord","target":"Ord"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl UnorderedPair


         .. rubric:: Functions


         .. rust:function:: imctk_util::unordered_pair::UnorderedPair::into_values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_values"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"["},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"; "},{"type":"literal","value":"2"},{"type":"punctuation","value":"]"}]

            Returns the two elements as a sorted array.

         .. rust:function:: imctk_util::unordered_pair::UnorderedPair::map
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"map"},{"type":"punctuation","value":"<"},{"type":"name","value":"U"},{"type":"punctuation","value":": "},{"type":"link","value":"Ord","target":"Ord"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"U","target":"U"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"U","target":"U"},{"type":"punctuation","value":">"}]

            Applies a function to the two elements, returning the results as a new [`UnorderedPair`].

         .. rust:function:: imctk_util::unordered_pair::UnorderedPair::max_element
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"max_element"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"}]

            Returns a reference to the larger of the two elements.

         .. rust:function:: imctk_util::unordered_pair::UnorderedPair::min_element
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"min_element"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"}]

            Returns a reference to the smaller of the two elements.

         .. rust:function:: imctk_util::unordered_pair::UnorderedPair::new
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"new"},{"type":"punctuation","value":"("},{"type":"name","value":"values"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"["},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"; "},{"type":"literal","value":"2"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates a new unordered pair by sorting two values.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_util::unordered_pair::UnorderedPair::Deref
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ops","target":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"Deref"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Deref for UnorderedPair


      .. rust:impl:: imctk_util::unordered_pair::UnorderedPair::From
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Ord","target":"Ord"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"From","target":"From"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"["},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"; "},{"type":"literal","value":"2"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl From for UnorderedPair


      .. rust:impl:: imctk_util::unordered_pair::UnorderedPair::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"UnorderedPair","target":"UnorderedPair"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for UnorderedPair

