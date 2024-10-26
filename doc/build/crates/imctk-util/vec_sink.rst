================
``mod vec_sink``
================


.. rust:module:: imctk_util::vec_sink
   :index: 0
   :vis: pub


   .. rust:use:: imctk_util::vec_sink
      :used_name: self


   .. rust:use:: std::ops::Deref
      :used_name: Deref


   .. rust:use:: std::ops::DerefMut
      :used_name: DerefMut


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_util::vec_sink::VecSink
      :index: 1
      :vis: pub
      :toc: struct VecSink
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"VecSink"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      An append only wrappers for a mutable [`Vec`] reference.
      
      This remembers the vector's length at creation and will not allow modification or deletion
      of any element that was already present at creation.
      
      The [`Deref`] and [`DerefMut`] implementations provide access to the slice of newly added
      elements.

      .. rubric:: Implementations


      .. rust:impl:: imctk_util::vec_sink::VecSink
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl VecSink


         .. rubric:: Functions


         .. rust:function:: imctk_util::vec_sink::VecSink::append
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"append"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"values"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Append all elements drained from the given vector.
            
            This forwards to [`Vec::append`].

         .. rust:function:: imctk_util::vec_sink::VecSink::borrow_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"borrow_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Re-borrows the existing [`VecSink`].

         .. rust:function:: imctk_util::vec_sink::VecSink::borrow_sink
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"borrow_sink"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Re-borrows as a new [`VecSink`] hiding and protecting any elements already added.

         .. rust:function:: imctk_util::vec_sink::VecSink::dedup
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dedup"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"}]

            Implements [`Vec::dedup`] limited to the newly added elements.
            
            Note that this does not compare the first newly added element with its predecessor in the
            underlying `Vec`.

         .. rust:function:: imctk_util::vec_sink::VecSink::new
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"new"},{"type":"punctuation","value":"("},{"type":"name","value":"target"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates an append only wrapper that ensures the currently present elements will be
            preserved.

         .. rust:function:: imctk_util::vec_sink::VecSink::push
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"push"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"}]

            Append a new element.
            
            This forwards to [`Vec::push`].

         .. rust:function:: imctk_util::vec_sink::VecSink::truncate
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"truncate"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"len"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Implements [`Vec::truncate`] limited to the newly added elements.
            
            The given length referes to the number of newly added elements to keep, not to the number of
            elements in the underlying `Vec`.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_util::vec_sink::VecSink::Deref
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Deref","target":"Deref"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Deref for VecSink


      .. rust:impl:: imctk_util::vec_sink::VecSink::DerefMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"DerefMut","target":"DerefMut"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl DerefMut for VecSink


      .. rust:impl:: imctk_util::vec_sink::VecSink::Extend
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"A"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Extend","target":"Extend"},{"type":"punctuation","value":"<"},{"type":"link","value":"A","target":"A"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"A","target":"A"},{"type":"punctuation","value":">"}]
         :toc: impl Extend for VecSink

