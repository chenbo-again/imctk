=================
``mod give_take``
=================


.. rust:module:: imctk_util::give_take
   :index: 0
   :vis: pub


   .. rust:use:: imctk_util::give_take
      :used_name: self


   .. rust:use:: std::ops::Deref
      :used_name: Deref


   .. rust:use:: std::ops::DerefMut
      :used_name: DerefMut


   .. rust:use:: std::mem::ManuallyDrop
      :used_name: ManuallyDrop
      :reexport: imctk_util::give_take


   .. rust:use:: imctk_util::give
      :used_name: give
      :reexport: imctk_util::give_take

   .. rubric:: Re-exports

   * :rust:any:`std::mem::ManuallyDrop`
   * :rust:any:`imctk_util::give`

   .. rubric:: Macros


   .. rust:macro:: imctk_util::give_take::give
      :index: 0
      :vis: pub

      Safely constructs an ownership-transferring [`Take`] reference.

   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_util::give_take::Take
      :index: 1
      :vis: pub
      :toc: struct Take
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Take"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"ManuallyDrop","target":"ManuallyDrop"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

      An ownership-transferring mutable reference.

      .. rubric:: Implementations


      .. rust:impl:: imctk_util::give_take::Take
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Take","target":"Take"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Take


         .. rubric:: Functions


         .. rust:function:: imctk_util::give_take::Take::take
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"take"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"}]

            Returns the referenced value, taking ownership and moving it out of the referenced
            storage.

      .. rust:impl:: imctk_util::give_take::Take
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Take","target":"Take"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Take


         .. rubric:: Functions


         .. rust:function:: imctk_util::give_take::Take::from_raw_ptr
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_raw_ptr"},{"type":"punctuation","value":"("},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"operator","value":"*"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Takes a value from a mutable raw pointer.
            
            # Safety
            This takes ownership of the target value, but keeps using the pointed to storage. Thus
            callers must ensure that the pointed to storage remains allocated and untouched until
            the [`Take`] value is consumed.

         .. rust:function:: imctk_util::give_take::Take::into_raw_ptr
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_raw_ptr"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"operator","value":"*"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"}]

            Obtains a raw pointer to the taken value, transferring ownership to the caller.

         .. rust:function:: imctk_util::give_take::Take::new
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"new"},{"type":"punctuation","value":"("},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"ManuallyDrop","target":"ManuallyDrop"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Takes a value by reference from a mutable [`ManuallyDrop`] reference.
            
            # Safety
            This takes ownership of the passed value and thus has the same safety requirements as
            [`ManuallyDrop::take`].

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_util::give_take::Take::Deref
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Deref","target":"Deref"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Take","target":"Take"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Deref for Take


      .. rust:impl:: imctk_util::give_take::Take::DerefMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"DerefMut","target":"DerefMut"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Take","target":"Take"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl DerefMut for Take


      .. rust:impl:: imctk_util::give_take::Take::Drop
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Drop","target":"Drop"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Take","target":"Take"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Drop for Take

