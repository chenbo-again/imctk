===================
Crate ``imctk_ids``
===================


.. rust:crate:: imctk_ids
   :index: 0

   Type checked and niche compatible integer ids.

   .. rubric:: Modules
   .. toctree::
      :maxdepth: 1

      id_vec
      indexed_id_vec
      id_set_seq


   .. rust:use:: imctk_ids
      :used_name: self


   .. rust:use:: core::hash::Hash
      :used_name: Hash


   .. rust:use:: imctk_derive::Id
      :used_name: Id
      :reexport: imctk_ids


   .. rust:use:: imctk_ids::Id16
      :used_name: Id16


   .. rust:use:: imctk_ids::Id64
      :used_name: Id64


   .. rust:use:: imctk_ids::ConstIdFromIdIndex
      :used_name: ConstIdFromIdIndex


   .. rust:use:: imctk_ids::GenericId
      :used_name: GenericId


   .. rust:use:: imctk_ids::Id8
      :used_name: Id8


   .. rust:use:: imctk_ids::Id32
      :used_name: Id32


   .. rust:use:: imctk_ids::IdSize
      :used_name: IdSize


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: imctk_ids::IdRange
      :used_name: IdRange

   .. rubric:: Re-exports

   * :rust:any:`imctk_derive::Id`
     Derives an [`Id`] instance for a newtype wrapper around an existing [`Id`] type.
     
     Deriving an [`Id`] instance requires the `#[repr(transparent)]` attribute on the target struct.
     
     This automatically derives all of [`Id`]'s supertraits ([`Clone`], [`Copy`], [`PartialEq`],
     [`Eq`], [`PartialOrd`], [`Ord`] , [`Hash`], [`Send`] and [`Sync`]) to ensure those traits are
     implemented according to `Id`'s safety requirements.

   .. rubric:: Traits


   .. rust:trait:: imctk_ids::ConstIdFromIdIndex
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"}]

      Used to construct constant values from an index.
      
      Together with [`Id::FromConstIdIndex`], this works around the lack of support for `const fn` in
      traits.
      
      

      .. rubric:: Types


      .. rust:type:: imctk_ids::ConstIdFromIdIndex::Id
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"Id"}]

         This trait is usually implemented on the target [`Id`] type, in which case this can be `Self`.

      .. rubric:: Variables


      .. rust:variable:: imctk_ids::ConstIdFromIdIndex::ID
         :index: -1
         :vis: pub
         :toc: const ID
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"ID"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Id"}]

         The constant ID value having the index `INDEX`.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ids::id::Id8::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl ConstIdFromIdIndex for Id8


      .. rust:impl:: imctk_ids::id::Id16::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id16","target":"Id16"}]
         :toc: impl ConstIdFromIdIndex for Id16


      .. rust:impl:: imctk_ids::id::Id32::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id32","target":"Id32"}]
         :toc: impl ConstIdFromIdIndex for Id32


      .. rust:impl:: imctk_ids::id::Id64::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id64","target":"Id64"}]
         :toc: impl ConstIdFromIdIndex for Id64


      .. rust:impl:: imctk_ids::id::IdSize::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSize","target":"IdSize"}]
         :toc: impl ConstIdFromIdIndex for IdSize


      .. rust:impl:: imctk_ids::id::usize::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]
         :toc: impl ConstIdFromIdIndex for usize


      .. rust:impl:: imctk_ids::id::u64::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]
         :toc: impl ConstIdFromIdIndex for u64


      .. rust:impl:: imctk_ids::id::u32::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u32","target":"u32"}]
         :toc: impl ConstIdFromIdIndex for u32


      .. rust:impl:: imctk_ids::id::u16::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u16","target":"u16"}]
         :toc: impl ConstIdFromIdIndex for u16


      .. rust:impl:: imctk_ids::id::u8::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u8","target":"u8"}]
         :toc: impl ConstIdFromIdIndex for u8


   .. rust:trait:: imctk_ids::Id
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"Id"}]

      Types that represent integer ids
      
      A type of this trait represents an `usize` index value in the range `0..=Self::MAX_ID_INDEX`,
      with the specific representation used is up to the implementing type. Implementing types can
      have a smaller size or alignment compared to `usize`, and can leave space for rustc's
      niche-value optimization.
      
      # Safety
      This trait comes with several safety critical requirements for implementing types:
      
      * The [`Clone`] implementation must return a [`Copy`].
      * The [`PartialEq`], [`Eq`], [`PartialOrd`] and [`Ord`] implementation must behave as if this
        type was a `struct` containing [`index: usize`][`Self::id_index()`] as only field and a
        `#[derive(PartialEq, Eq, PartialOrd, Ord)]` attribute.
      * Each valid index must have a unique representation , i.e. it must be possible to check two ids
        for equality by comparing their raw bytes. This only applies to equality, and does not extend
        to [`PartialOrd`] or [`Ord`].
      * The [`Hash`] implementation must be deterministic and only depend on the represented index.
      * The associated [`Self::BaseId`] type must be an [`Id`] with the same representation and index
        range (it can be `Self`).
      * The associated [`Self::GenericId`] type must be
        [`GenericId<{Self::MAX_ID_INDEX}>`][`GenericId`].
      * All implemented trait items must confirm to the their documented behavior.
      
      Users of this trait may depend on implementing types following these requirements for upholding
      their own safety invariants.
      
      

      .. rubric:: Types


      .. rust:type:: imctk_ids::Id::BaseId
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"BaseId"}]

         An [`Id`] type that has the same representation and index range as this type.
         
         This is provided to enable writing generic code that during monomorphization is only
         instantiated once per base type instead of once per id type.
         
         For types that are not newtype wrappers around an existing id type, this is usually
         [`Self`].

      .. rust:type:: imctk_ids::Id::FromConstIdIndex
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"FromConstIdIndex"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"}]

         Used to construct constant values from an index.
         
         This works around the lack of support for `const fn` in traits.

      .. rust:type:: imctk_ids::Id::GenericId
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"GenericId"}]

         The [`GenericId`] type, parametrized to have the same index range as this type.
         
         This is used for type checked conversions between ids that do not have to share the same
         representation.

      .. rubric:: Variables


      .. rust:variable:: imctk_ids::Id::MAX_ID
         :index: -1
         :vis: pub
         :toc: const MAX_ID
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"}]

         The id with the largest representable index.

      .. rust:variable:: imctk_ids::Id::MAX_ID_INDEX
         :index: -1
         :vis: pub
         :toc: const MAX_ID_INDEX
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"}]

         The largest index representable by this id type.

      .. rust:variable:: imctk_ids::Id::MIN_ID
         :index: -1
         :vis: pub
         :toc: const MIN_ID
         :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MIN_ID"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"}]

         The id with index zero.

      .. rubric:: Functions


      .. rust:function:: imctk_ids::Id::cast_from_id
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"cast_from_id"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":"<"},{"type":"name","value":"GenericId"},{"type":"punctuation","value":" = "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"GenericId"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"name","value":"from"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

         Creates a value from a compatible [`Id`] type using the same index.
         
         Two id types are considered compatible when the have the same [`Self::BaseId`] type.
         
         As compatible types can represent the same range of indices, this cannot fail and will never
         panic.

      .. rust:function:: imctk_ids::Id::cast_into_id
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"cast_into_id"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":"<"},{"type":"name","value":"GenericId"},{"type":"punctuation","value":" = "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"GenericId"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"}]

         Returns a value of a compatible [`Id`] type having the same index.
         
         Two id types are considered compatible when the have the same [`Self::BaseId`] type.
         
         As compatible types can represent the same range of indices, this cannot fail and will never
         panic.

      .. rust:function:: imctk_ids::Id::from_base_id
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_base_id"},{"type":"punctuation","value":"("},{"type":"name","value":"base"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"BaseId"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

         Returns an id with the same index as a [`Self::BaseId`] id.
         
         This cannot fail and will never panic.

      .. rust:function:: imctk_ids::Id::from_id_index
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_id_index"},{"type":"punctuation","value":"("},{"type":"name","value":"index"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

         Returns the id with a given index, panicking when the index is invalid.
         
         This panics if and only if `index > Self::MAX_ID_INDEX`.

      .. rust:function:: imctk_ids::Id::from_id_index_unchecked
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_id_index_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"index"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

         Returns the id with a given index, assuming a valid index.
         
         # Safety
         This is only safe to call when `index <= Self::MAX_ID_INDEX`, which is not checked by this
         method.
         
         Implementations are encouraged to include a debug-only assertion for this requirement.

      .. rust:function:: imctk_ids::Id::id_index
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"id_index"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

         Returns the index represented by this id.

      .. rust:function:: imctk_ids::Id::into_base_id
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_base_id"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"BaseId"}]

         Returns a [`Self::BaseId`] id of the same index.
         
         This cannot fail and will never panic.

      .. rust:function:: imctk_ids::Id::try_from_id_index
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"try_from_id_index"},{"type":"punctuation","value":"("},{"type":"name","value":"index"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":">"}]

         Returns the id with a given index, if it is valid.
         
         This returns `None` if and only if `index > Self::MAX_ID_INDEX`.
         
         Never panics.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ids::id::Id8::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl Id for Id8


      .. rust:impl:: imctk_ids::id::Id16::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id16","target":"Id16"}]
         :toc: impl Id for Id16


      .. rust:impl:: imctk_ids::id::Id32::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id32","target":"Id32"}]
         :toc: impl Id for Id32


      .. rust:impl:: imctk_ids::id::Id64::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id64","target":"Id64"}]
         :toc: impl Id for Id64


      .. rust:impl:: imctk_ids::id::IdSize::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSize","target":"IdSize"}]
         :toc: impl Id for IdSize


      .. rust:impl:: imctk_ids::id::usize::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]
         :toc: impl Id for usize


      .. rust:impl:: imctk_ids::id::u64::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]
         :toc: impl Id for u64


      .. rust:impl:: imctk_ids::id::u32::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u32","target":"u32"}]
         :toc: impl Id for u32


      .. rust:impl:: imctk_ids::id::u16::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u16","target":"u16"}]
         :toc: impl Id for u16


      .. rust:impl:: imctk_ids::id::u8::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"u8","target":"u8"}]
         :toc: impl Id for u8


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ids::GenericId
      :index: 1
      :vis: pub
      :toc: struct GenericId
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"GenericId"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"link","value":"Repr","target":"Repr"},{"type":"punctuation","value":")"}]

      A newtype wrapper around an `Id` type that limits the contained value to the `MAX_ID_INDEX`
      generic parameter.
      
      Used with `usize` as `Id` type to implement conversion between [`Id`] types that have different
      representations but that share the same index range.
      
      

      .. rubric:: Implementations


      .. rust:impl:: imctk_ids::id::GenericId
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"GenericId","target":"GenericId"},{"type":"punctuation","value":"<"},{"type":"link","value":"MAX_ID_INDEX","target":"MAX_ID_INDEX"},{"type":"punctuation","value":", "},{"type":"link","value":"Repr","target":"Repr"},{"type":"punctuation","value":">"}]
         :toc: impl GenericId


         .. rubric:: Functions


         .. rust:function:: imctk_ids::id::GenericId::into_generic_id_repr
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_generic_id_repr"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Repr","target":"Repr"}]

            Returns a `Repr` id of the same index.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ids::id::GenericId::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"GenericId","target":"GenericId"},{"type":"punctuation","value":"<"},{"type":"link","value":"MAX_ID_INDEX","target":"MAX_ID_INDEX"},{"type":"punctuation","value":", "},{"type":"link","value":"Repr","target":"Repr"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for GenericId


      .. rust:impl:: imctk_ids::id::GenericId::ConstIdFromIdIndex
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ConstIdFromIdIndex","target":"ConstIdFromIdIndex"},{"type":"punctuation","value":"<"},{"type":"link","value":"INDEX","target":"INDEX"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"GenericId","target":"GenericId"},{"type":"punctuation","value":"<"},{"type":"link","value":"MAX_ID_INDEX","target":"MAX_ID_INDEX"},{"type":"punctuation","value":", "},{"type":"link","value":"Repr","target":"Repr"},{"type":"punctuation","value":">"}]
         :toc: impl ConstIdFromIdIndex for GenericId


      .. rust:impl:: imctk_ids::id::GenericId::Id
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Id","target":"Id"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"GenericId","target":"GenericId"},{"type":"punctuation","value":"<"},{"type":"link","value":"MAX_ID_INDEX","target":"MAX_ID_INDEX"},{"type":"punctuation","value":", "},{"type":"link","value":"Repr","target":"Repr"},{"type":"punctuation","value":">"}]
         :toc: impl Id for GenericId


      .. rust:impl:: imctk_ids::id::GenericId::Send
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Send","target":"Send"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"GenericId","target":"GenericId"},{"type":"punctuation","value":"<"},{"type":"link","value":"MAX_ID_INDEX","target":"MAX_ID_INDEX"},{"type":"punctuation","value":", "},{"type":"link","value":"Repr","target":"Repr"},{"type":"punctuation","value":">"}]
         :toc: impl Send for GenericId


      .. rust:impl:: imctk_ids::id::GenericId::Sync
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"keyword","value":"const"},{"type":"space"},{"type":"name","value":"MAX_ID_INDEX"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"Repr"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Sync","target":"Sync"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"GenericId","target":"GenericId"},{"type":"punctuation","value":"<"},{"type":"link","value":"MAX_ID_INDEX","target":"MAX_ID_INDEX"},{"type":"punctuation","value":", "},{"type":"link","value":"Repr","target":"Repr"},{"type":"punctuation","value":">"}]
         :toc: impl Sync for GenericId


   .. rust:struct:: imctk_ids::Id16
      :index: 1
      :vis: pub
      :toc: struct Id16
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Id16"}]

      
      

   .. rust:struct:: imctk_ids::Id32
      :index: 1
      :vis: pub
      :toc: struct Id32
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Id32"}]

      
      

   .. rust:struct:: imctk_ids::Id64
      :index: 1
      :vis: pub
      :toc: struct Id64
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Id64"}]

      
      

   .. rust:struct:: imctk_ids::Id8
      :index: 1
      :vis: pub
      :toc: struct Id8
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Id8"},{"type":"punctuation","value":"("},{"type":"link","value":"NonMaxHighNibbleU8","target":"NonMaxHighNibbleU8"},{"type":"punctuation","value":")"}]

      [`Id`] type representing indices in the range `0..0xf0`.
      
      

      .. rubric:: Implementations


      .. rust:impl:: imctk_ids::id::id_types::id8::Id8
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl Id8


         .. rubric:: Functions


         .. rust:function:: imctk_ids::id::id_types::id8::Id8::from_index_const
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"const"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_index_const"},{"type":"punctuation","value":"("},{"type":"name","value":"index"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Returns the id with a given index, panicking when the index is invalid.
            
            Unlike the [`Id::from_id_index`] this is a `const fn`.
            
            This panics if and only if `index > Self::MAX_ID_INDEX`.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ids::id::id_types::id8::Id8::SubtypeCast
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"SubtypeCast","target":"SubtypeCast"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl SubtypeCast for Id8


      .. rust:impl:: imctk_ids::id::id_types::id8::Id8::PartialEq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"PartialEq","target":"PartialEq"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl PartialEq for Id8


      .. rust:impl:: imctk_ids::id::id_types::id8::Id8::Eq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Eq","target":"Eq"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl Eq for Id8


      .. rust:impl:: imctk_ids::id::id_types::id8::Id8::PartialOrd
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"PartialOrd","target":"PartialOrd"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl PartialOrd for Id8


      .. rust:impl:: imctk_ids::id::id_types::id8::Id8::Ord
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Ord","target":"Ord"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl Ord for Id8


      .. rust:impl:: imctk_ids::id::id_types::id8::Id8::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl Debug for Id8


      .. rust:impl:: imctk_ids::id::id_types::id8::Id8::Hash
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Hash","target":"Hash"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Id8","target":"Id8"}]
         :toc: impl Hash for Id8


   .. rust:struct:: imctk_ids::IdRange
      :index: 1
      :vis: pub
      :toc: struct IdRange
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"IdRange"},{"type":"punctuation","value":"<"},{"type":"name","value":"I"},{"type":"punctuation","value":">"}]

      A range of [`Id`] values having contiguous indices.
      
      

      .. rubric:: Implementations


      .. rust:impl:: imctk_ids::id_range::IdRange
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"I"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IdRange","target":"IdRange"},{"type":"punctuation","value":"<"},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":">"}]
         :toc: impl IdRange


         .. rubric:: Functions


         .. rust:function:: imctk_ids::id_range::IdRange::from_index_range
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_index_range"},{"type":"punctuation","value":"("},{"type":"name","value":"range"},{"type":"punctuation","value":": "},{"type":"link","value":"Range","target":"Range"},{"type":"punctuation","value":"<"},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates an id range given a corresponding index range.
            
            # Panics
            
            Panics when the range contains indices that are not valid for `I`.

         .. rust:function:: imctk_ids::id_range::IdRange::from_index_range_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_index_range_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"range"},{"type":"punctuation","value":": "},{"type":"link","value":"Range","target":"Range"},{"type":"punctuation","value":"<"},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates an id range given a corresponding index range without bounds checking.
            
            # Safety
            
            The caller must ensure that the range contains only valid indices for `I`.

         .. rust:function:: imctk_ids::id_range::IdRange::indices
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"indices"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Range","target":"Range"},{"type":"punctuation","value":"<"},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":">"}]

            Returns the indices present in the id range.

         .. rust:function:: imctk_ids::id_range::IdRange::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` if the id range contains no ids.

         .. rust:function:: imctk_ids::id_range::IdRange::iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"::"},{"type":"name","value":"IntoIter"}]

            Returns an iterator over the ids in the range.

         .. rust:function:: imctk_ids::id_range::IdRange::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of ids in the id range.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ids::id_range::IdRange::From
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"I"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"From","target":"From"},{"type":"punctuation","value":"<"},{"type":"link","value":"Range","target":"Range"},{"type":"punctuation","value":"<"},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdRange","target":"IdRange"},{"type":"punctuation","value":"<"},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":">"}]
         :toc: impl From for IdRange


      .. rust:impl:: imctk_ids::id_range::IdRange::IntoIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"I"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdRange","target":"IdRange"},{"type":"punctuation","value":"<"},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":">"}]
         :toc: impl IntoIterator for IdRange


   .. rust:struct:: imctk_ids::IdSize
      :index: 1
      :vis: pub
      :toc: struct IdSize
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"IdSize"}]

      
      
