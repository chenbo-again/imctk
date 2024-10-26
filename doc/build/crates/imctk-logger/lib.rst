======================
Crate ``imctk_logger``
======================


.. rust:crate:: imctk_logger
   :index: 0

   Default logging setup for the Incremental Model Checking Toolkit

   .. rust:use:: imctk_logger
      :used_name: self


   .. rust:use:: std::sync::atomic::AtomicBool
      :used_name: AtomicBool


   .. rust:use:: std::cell::RefCell
      :used_name: RefCell


   .. rust:use:: std::fmt
      :used_name: fmt


   .. rust:use:: std::sync::atomic::AtomicUsize
      :used_name: AtomicUsize


   .. rubric:: Functions


   .. rust:function:: imctk_logger::setup
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"setup"},{"type":"punctuation","value":"("},{"type":"punctuation","value":")"}]

      Perform the default logging setup used by imctk examples

   .. rust:function:: imctk_logger::test_setup
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"test_setup"},{"type":"punctuation","value":"("},{"type":"name","value":"config"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"str","target":"str"},{"type":"punctuation","value":")"}]

      Perform the logging setup using a given default log config
