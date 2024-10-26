===========================
Crate ``imctk_transparent``
===========================


.. rust:crate:: imctk_transparent
   :index: 0

   Safe casts for `repr(transparent)` structs.

   .. rust:use:: imctk_transparent
      :used_name: self


   .. rust:use:: std::mem::ManuallyDrop
      :used_name: ManuallyDrop


   .. rust:use:: std::ptr::NonNull
      :used_name: NonNull


   .. rust:use:: imctk_derive::NewtypeCast
      :used_name: NewtypeCast
      :reexport: imctk_transparent


   .. rust:use:: imctk_derive::SubtypeCast
      :used_name: SubtypeCast
      :reexport: imctk_transparent

   .. rubric:: Re-exports

   * :rust:any:`imctk_derive::NewtypeCast`
     Derives a [`NewtypeCast`] instance for a `repr(transparent)` struct.
   * :rust:any:`imctk_derive::SubtypeCast`
     Derives a [`SubtypeCast`] instance for a `repr(transparent)` struct.

   .. rubric:: Traits


   .. rust:trait:: imctk_transparent::NewtypeCast
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"NewtypeCast"}]

      Provides mutable access to a `repr(transparent)` struct's data.
      
      As this provides mutable access, it can only be used for structs that are safe to use with
      arbitrary wrapped values. If this is not the case, only [`SubtypeCast`] can be implemented.
      
      # Safety
      
      This may only be implemented if all values of the implementing type can be safely transmuted
      into [`<Self as SubtypeCast>::Repr`][`SubtypeCast::Repr`] values and if all `<Self as
      SubtypeCast>::Repr` values can be safely transmuted back into the implementing type.
      
      For `repr(transparent)` structs, this trait can be safely derived.

      .. rubric:: Functions


      .. rust:function:: imctk_transparent::NewtypeCast::as_repr_mut
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"as_repr_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"}]

         Casts a mutable reference to a `repr(transparent)` wrapper into a mutable reference to
         its inner type.

      .. rust:function:: imctk_transparent::NewtypeCast::from_repr
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_repr"},{"type":"punctuation","value":"("},{"type":"name","value":"repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"}]

         Casts a value of an inner type into a value of a `repr(transparent)` wrapper.

      .. rust:function:: imctk_transparent::NewtypeCast::from_repr_mut
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_repr_mut"},{"type":"punctuation","value":"("},{"type":"name","value":"repr"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

         Casts a mutable reference to an inner type into a mutable reference of a `repr(transparent)`
         wrapper.

      .. rust:function:: imctk_transparent::NewtypeCast::from_repr_ref
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_repr_ref"},{"type":"punctuation","value":"("},{"type":"name","value":"repr"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"}]

         Casts a reference to an inner type into a reference of a `repr(transparent)` wrapper.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_transparent::*T::NewtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"operator","value":"*"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl NewtypeCast for *T


      .. rust:impl:: imctk_transparent::*T::NewtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"operator","value":"*"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl NewtypeCast for *T


      .. rust:impl:: imctk_transparent::NonNull::NewtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"NonNull","target":"NonNull"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl NewtypeCast for NonNull


      .. rust:impl:: imctk_transparent::Option::NewtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"NonNull","target":"NonNull"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]
         :toc: impl NewtypeCast for Option


      .. rust:impl:: imctk_transparent::&T::NewtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl NewtypeCast for &T


      .. rust:impl:: imctk_transparent::&T::NewtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl NewtypeCast for &T


      .. rust:impl:: imctk_transparent::[T]::NewtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"["},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"]"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"}]
         :toc: impl NewtypeCast for [T]


      .. rust:impl:: imctk_transparent::[T; ?]::NewtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":", "},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"N"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"["},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"; "},{"type":"punctuation","value":"?"},{"type":"punctuation","value":"]"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"}]
         :toc: impl NewtypeCast for [T; ?]


   .. rust:trait:: imctk_transparent::SubtypeCast
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"SubtypeCast"}]

      Provides read-only access to a `repr(transparent)` struct's data.
      
      As this provides read-only access, this can be used for structs that are only safe to use with
      some possible wrapped values, i.e. that behave somewhat like a subtype of the wrapped type.
      
      # Safety
      
      This may only be implemented if all values of `Self` can be safely transmuted into
      [`Self::Repr`] values. Additionally it must be safe to transmute a `*mut Self` into a `*mut
      Self::Repr`, i.e. they must have compatible pointer metadata.
      
      For `repr(transparent)` structs, this trait can be safely derived.

      .. rubric:: Types


      .. rust:type:: imctk_transparent::SubtypeCast::Repr
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"Repr"}]

         Type of the `repr(transparent)` struct's only field.

      .. rubric:: Variables


      .. rust:variable:: imctk_transparent::SubtypeCast::__STATIC_ASSERT_HELPER
         :index: -1
         :vis: pub
         :toc: const __STATIC_ASSERT_HELPER
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"__STATIC_ASSERT_HELPER"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"("},{"type":"punctuation","value":")"}]


      .. rubric:: Functions


      .. rust:function:: imctk_transparent::SubtypeCast::as_repr
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"as_repr"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"}]

         Casts a reference to a `repr(transparent)` wrapper into a reference to its inner type.

      .. rust:function:: imctk_transparent::SubtypeCast::as_repr_mut_unchecked
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"as_repr_mut_unchecked"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"}]

         Casts a mutable reference to a `repr(transparent)` wrapper into a mutable reference to
         its inner type without checking invariants.
         
         # Safety
         The caller must ensure that the subtype has fully documented invariants and that these are
         upheld whenever the lifetime of the returned value ends. In particular this includes cases
         where the lifetime ends due to panic-unwinding.

      .. rust:function:: imctk_transparent::SubtypeCast::from_repr_mut_unchecked
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_repr_mut_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"repr"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

         Casts a mutable reference to an inner type into a mutable reference of a
         `repr(transparent)` wrapper without checking invariants.
         
         # Safety
         The caller must ensure that the subtype has fully documented invariants and that these are
         upheld by the provided reference.

      .. rust:function:: imctk_transparent::SubtypeCast::from_repr_ref_unchecked
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_repr_ref_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"repr"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"}]

         Casts a reference to an inner type into a reference of a `repr(transparent)` wrapper
         without checking invariants.
         
         # Safety
         The caller must ensure that the subtype has fully documented invariants and that these are
         upheld by the provided reference.

      .. rust:function:: imctk_transparent::SubtypeCast::from_repr_unchecked
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_repr_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"}]

         Casts a mutable reference to a `repr(transparent)` wrapper into a mutable reference to
         its inner type without checking invariants.
         
         # Safety
         The caller must ensure that the subtype has fully documented invariants and that these are
         upheld by the provided value.

      .. rust:function:: imctk_transparent::SubtypeCast::into_repr
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_repr"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"}]

         Casts a value of a `repr(transparent)` wrapper into its inner type.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_transparent::*T::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"operator","value":"*"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl SubtypeCast for *T


      .. rust:impl:: imctk_transparent::*T::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"operator","value":"*"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl SubtypeCast for *T


      .. rust:impl:: imctk_transparent::NonNull::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"NonNull","target":"NonNull"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl SubtypeCast for NonNull


      .. rust:impl:: imctk_transparent::Option::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"NonNull","target":"NonNull"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]
         :toc: impl SubtypeCast for Option


      .. rust:impl:: imctk_transparent::&T::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl SubtypeCast for &T


      .. rust:impl:: imctk_transparent::&T::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"NewtypeCast","target":"NewtypeCast"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"}]
         :toc: impl SubtypeCast for &T


      .. rust:impl:: imctk_transparent::[T]::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"["},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"]"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"}]
         :toc: impl SubtypeCast for [T]


      .. rust:impl:: imctk_transparent::[T; ?]::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"punctuation","value":", "},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"N"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"punctuation","value":"["},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"; "},{"type":"punctuation","value":"?"},{"type":"punctuation","value":"]"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Sized","target":"Sized"}]
         :toc: impl SubtypeCast for [T; ?]

