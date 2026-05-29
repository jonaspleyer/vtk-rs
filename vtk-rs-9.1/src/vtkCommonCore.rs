pub trait VtkAOSDataArrayTemplate {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_typed_tuple(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        tuple: *mut core::ffi::c_void,
    ) -> ();
    fn write_pointer(
        &mut self,
        valueIdx: core::ffi::c_longlong,
        numValues: core::ffi::c_longlong,
    ) -> *mut core::ffi::c_void;
    fn get_pointer(&mut self, valueIdx: core::ffi::c_longlong) -> *mut core::ffi::c_void;
    fn set_array(
        &mut self,
        array: *mut core::ffi::c_void,
        size: core::ffi::c_longlong,
        save: core::ffi::c_int,
        deleteMethod: core::ffi::c_int,
    ) -> ();
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> ();
    fn data_element_changed(&mut self, p0: core::ffi::c_longlong) -> ();
    fn begin(&mut self) -> *mut core::ffi::c_void;
    fn end(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkAbstractArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn allocate(
        &mut self,
        numValues: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn get_element_component_size(&mut self) -> core::ffi::c_int;
    fn set_number_of_components(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_components_min_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_components_max_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_components(&mut self) -> core::ffi::c_int;
    fn set_component_name(&mut self, component: core::ffi::c_longlong, name: &str) -> ();
    fn get_component_name(&mut self, component: core::ffi::c_longlong) -> &str;
    fn has_a_component_name(&mut self) -> bool;
    fn copy_component_names(&mut self, da: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn set_number_of_tuples(&mut self, numTuples: core::ffi::c_longlong) -> ();
    fn set_number_of_values(&mut self, numValues: core::ffi::c_longlong) -> bool;
    fn get_number_of_tuples(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_values(&mut self) -> core::ffi::c_longlong;
    fn set_tuple(
        &mut self,
        dstTupleIdx: core::ffi::c_longlong,
        srcTupleIdx: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuple(
        &mut self,
        dstTupleIdx: core::ffi::c_longlong,
        srcTupleIdx: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuples(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_next_tuple(
        &mut self,
        srcTupleIdx: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong;
    fn get_tuples(
        &mut self,
        tupleIds: *mut core::ffi::c_void,
        output: *mut core::ffi::c_void,
    ) -> ();
    fn has_standard_memory_layout(&mut self) -> bool;
    fn deep_copy(&mut self, da: *mut core::ffi::c_void) -> ();
    fn squeeze(&mut self) -> ();
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int;
    fn reset(&mut self) -> ();
    fn get_size(&mut self) -> core::ffi::c_longlong;
    fn get_max_id(&mut self) -> core::ffi::c_longlong;
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> ();
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong;
    fn set_name(&mut self, _arg: &str) -> ();
    fn get_data_type_as_string(&mut self) -> &str;
    fn create_array(&mut self, dataType: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn is_numeric(&mut self) -> core::ffi::c_int;
    fn new_iterator(&mut self) -> *mut core::ffi::c_void;
    fn get_data_size(&mut self) -> core::ffi::c_longlong;
    fn data_changed(&mut self) -> ();
    fn clear_lookup(&mut self) -> ();
    fn get_prominent_component_values(
        &mut self,
        comp: core::ffi::c_int,
        values: *mut core::ffi::c_void,
        uncertainty: core::ffi::c_double,
        minimumProminence: core::ffi::c_double,
    ) -> ();
    fn get_information(&mut self) -> *mut core::ffi::c_void;
    fn has_information(&mut self) -> bool;
    fn copy_information(
        &mut self,
        infoFrom: *mut core::ffi::c_void,
        deep: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn gui_hide(&mut self) -> *mut core::ffi::c_void;
    fn per_component(&mut self) -> *mut core::ffi::c_void;
    fn per_finite_component(&mut self) -> *mut core::ffi::c_void;
    fn modified(&mut self) -> ();
    fn discrete_values(&mut self) -> *mut core::ffi::c_void;
    fn discrete_value_sample_parameters(&mut self) -> *mut core::ffi::c_void;
    fn get_max_discrete_values(&mut self) -> core::ffi::c_uint;
    fn set_max_discrete_values(&mut self, _arg: core::ffi::c_uint) -> ();
    fn get_array_type(&mut self) -> core::ffi::c_int;
}
pub trait VtkAnimationCue {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_time_mode(&mut self, mode: core::ffi::c_int) -> ();
    fn get_time_mode(&mut self) -> core::ffi::c_int;
    fn set_time_mode_to_relative(&mut self) -> ();
    fn set_time_mode_to_normalized(&mut self) -> ();
    fn set_start_time(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_start_time(&mut self) -> core::ffi::c_double;
    fn set_end_time(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_end_time(&mut self) -> core::ffi::c_double;
    fn tick(
        &mut self,
        currenttime: core::ffi::c_double,
        deltatime: core::ffi::c_double,
        clocktime: core::ffi::c_double,
    ) -> ();
    fn initialize(&mut self) -> ();
    fn finalize(&mut self) -> ();
    fn get_animation_time(&mut self) -> core::ffi::c_double;
    fn get_delta_time(&mut self) -> core::ffi::c_double;
    fn get_clock_time(&mut self) -> core::ffi::c_double;
}
pub trait VtkArchiver {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_archive_name(&mut self, _arg: &str) -> ();
    fn open_archive(&mut self) -> ();
    fn close_archive(&mut self) -> ();
    fn insert_into_archive(&mut self, relativePath: &str, data: &str, size: usize) -> ();
    fn contains(&mut self, relativePath: &str) -> bool;
}
pub trait VtkArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn create_array(
        &mut self,
        StorageType: core::ffi::c_int,
        ValueType: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn is_dense(&mut self) -> bool;
    fn resize(&mut self, i: core::ffi::c_longlong) -> ();
    fn get_dimensions(&mut self) -> core::ffi::c_longlong;
    fn get_size(&mut self) -> core::ffi::c_ulonglong;
    fn get_non_null_size(&mut self) -> core::ffi::c_ulonglong;
    fn deep_copy(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkArrayCoordinates {
    fn get_dimensions(&mut self) -> core::ffi::c_longlong;
    fn set_dimensions(&mut self, dimensions: core::ffi::c_longlong) -> ();
    fn get_coordinate(&mut self, i: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn set_coordinate(
        &mut self,
        i: core::ffi::c_longlong,
        p1: &core::ffi::c_longlong,
    ) -> ();
}
pub trait VtkArrayExtents {
    fn get_dimensions(&mut self) -> core::ffi::c_longlong;
    fn get_size(&mut self) -> core::ffi::c_ulonglong;
    fn set_dimensions(&mut self, dimensions: core::ffi::c_longlong) -> ();
    fn zero_based(&mut self) -> bool;
}
pub trait VtkArrayExtentsList {
    fn get_count(&mut self) -> core::ffi::c_longlong;
    fn set_count(&mut self, count: core::ffi::c_longlong) -> ();
}
pub trait VtkArrayIterator {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn initialize(&mut self, array: *mut core::ffi::c_void) -> ();
    fn get_data_type(&mut self) -> core::ffi::c_int;
}
pub trait VtkArrayIteratorTemplate {
    fn initialize(&mut self, array: *mut core::ffi::c_void) -> ();
    fn get_array(&mut self) -> *mut core::ffi::c_void;
    fn get_tuple(&mut self, id: core::ffi::c_longlong) -> *mut core::ffi::c_void;
    fn get_number_of_tuples(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_values(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_components(&mut self) -> core::ffi::c_int;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
}
pub trait VtkArrayRange {
    fn get_begin(&mut self) -> core::ffi::c_longlong;
    fn get_end(&mut self) -> core::ffi::c_longlong;
    fn get_size(&mut self) -> core::ffi::c_longlong;
}
pub trait VtkArraySort {
    fn get_dimensions(&mut self) -> core::ffi::c_longlong;
    fn set_dimensions(&mut self, dimensions: core::ffi::c_longlong) -> ();
}
pub trait VtkArrayWeights {
    fn get_count(&mut self) -> core::ffi::c_longlong;
    fn set_count(&mut self, count: core::ffi::c_longlong) -> ();
}
pub trait VtkBitArray {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn set_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_next_tuple(
        &mut self,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong;
    fn remove_tuple(&mut self, id: core::ffi::c_longlong) -> ();
    fn set_component(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_int,
        c: core::ffi::c_double,
    ) -> ();
    fn squeeze(&mut self) -> ();
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_int;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_int) -> ();
    fn insert_value(&mut self, id: core::ffi::c_longlong, i: core::ffi::c_int) -> ();
    fn insert_next_value(&mut self, i: core::ffi::c_int) -> core::ffi::c_longlong;
    fn insert_component(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_int,
        c: core::ffi::c_double,
    ) -> ();
    fn deep_copy(&mut self, da: *mut core::ffi::c_void) -> ();
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> ();
    fn new_iterator(&mut self) -> *mut core::ffi::c_void;
    fn data_changed(&mut self) -> ();
    fn clear_lookup(&mut self) -> ();
}
pub trait VtkBitArrayIterator {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn initialize(&mut self, array: *mut core::ffi::c_void) -> ();
    fn get_array(&mut self) -> *mut core::ffi::c_void;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_int;
    fn get_number_of_tuples(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_values(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_components(&mut self) -> core::ffi::c_int;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_int) -> ();
}
pub trait VtkBoxMuellerRandomSequence {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn initialize(&mut self, seed: core::ffi::c_uint) -> ();
    fn get_value(&mut self) -> core::ffi::c_double;
    fn next(&mut self) -> ();
    fn get_uniform_sequence(&mut self) -> *mut core::ffi::c_void;
    fn set_uniform_sequence(&mut self, uniformSequence: *mut core::ffi::c_void) -> ();
}
pub trait VtkBreakPoint {
    fn break_(&mut self) -> ();
}
pub trait VtkBuffer {
    fn get_buffer(&mut self) -> *mut core::ffi::c_void;
    fn set_buffer(
        &mut self,
        array: *mut core::ffi::c_void,
        size: core::ffi::c_longlong,
    ) -> ();
    fn set_malloc_function(&mut self, mallocFunction: *mut core::ffi::c_void) -> ();
    fn set_realloc_function(&mut self, reallocFunction: *mut core::ffi::c_void) -> ();
    fn set_free_function(
        &mut self,
        noFreeFunction: bool,
        deleteFunction: *mut core::ffi::c_void,
    ) -> ();
    fn get_size(&mut self) -> core::ffi::c_longlong;
    fn allocate(&mut self, size: core::ffi::c_longlong) -> bool;
    fn reallocate(&mut self, newsize: core::ffi::c_longlong) -> bool;
}
pub trait VtkByteSwap {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkCallbackCommand {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_callback(&mut self, f: *mut core::ffi::c_void) -> ();
    fn set_client_data_delete_callback(&mut self, f: *mut core::ffi::c_void) -> ();
    fn set_abort_flag_on_execute(&mut self, f: core::ffi::c_int) -> ();
    fn get_abort_flag_on_execute(&mut self) -> core::ffi::c_int;
    fn abort_flag_on_execute_on(&mut self) -> ();
    fn abort_flag_on_execute_off(&mut self) -> ();
}
pub trait VtkCharArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn set_typed_tuple(&mut self, i: core::ffi::c_longlong, tuple: &str) -> ();
    fn insert_typed_tuple(&mut self, i: core::ffi::c_longlong, tuple: &str) -> ();
    fn insert_next_typed_tuple(&mut self, tuple: &str) -> core::ffi::c_longlong;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> &str;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: &str) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: &str) -> ();
    fn insert_next_value(&mut self, f: &str) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> &str;
    fn get_data_type_value_max(&mut self) -> &str;
}
pub trait VtkCollection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn add_item(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn insert_item(&mut self, i: core::ffi::c_int, p1: *mut core::ffi::c_void) -> ();
    fn replace_item(&mut self, i: core::ffi::c_int, p1: *mut core::ffi::c_void) -> ();
    fn remove_item(&mut self, i: core::ffi::c_int) -> ();
    fn remove_all_items(&mut self) -> ();
    fn is_item_present(&mut self, a: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn get_number_of_items(&mut self) -> core::ffi::c_int;
    fn init_traversal(&mut self) -> ();
    fn get_next_item_as_object(&mut self) -> *mut core::ffi::c_void;
    fn get_item_as_object(&mut self, i: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn new_iterator(&mut self) -> *mut core::ffi::c_void;
    fn register(&mut self, o: *mut core::ffi::c_void) -> ();
}
pub trait VtkCollectionIterator {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_collection(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_collection(&mut self) -> *mut core::ffi::c_void;
    fn init_traversal(&mut self) -> ();
    fn go_to_first_item(&mut self) -> ();
    fn go_to_next_item(&mut self) -> ();
    fn is_done_with_traversal(&mut self) -> core::ffi::c_int;
    fn get_current_object(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkCommand {
    fn is_type_of(&mut self, type_: &str) -> core::ffi::c_int;
    fn is_a(&mut self, type_: &str) -> core::ffi::c_int;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_generations_from_base_type(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn get_number_of_generations_from_base(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn un_register(&mut self) -> ();
    fn get_string_from_event_id(&mut self, event: core::ffi::c_ulong) -> &str;
    fn get_event_id_from_string(&mut self, event: &str) -> core::ffi::c_ulong;
    fn event_has_data(&mut self, event: core::ffi::c_ulong) -> bool;
    fn set_abort_flag(&mut self, f: core::ffi::c_int) -> ();
    fn get_abort_flag(&mut self) -> core::ffi::c_int;
    fn abort_flag_on(&mut self) -> ();
    fn abort_flag_off(&mut self) -> ();
    fn set_passive_observer(&mut self, f: core::ffi::c_int) -> ();
    fn get_passive_observer(&mut self) -> core::ffi::c_int;
    fn passive_observer_on(&mut self) -> ();
    fn passive_observer_off(&mut self) -> ();
}
pub trait VtkCommonInformationKeyManager {
    fn register(&mut self, key: *mut core::ffi::c_void) -> ();
}
pub trait VtkConditionVariable {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn signal(&mut self) -> ();
    fn broadcast(&mut self) -> ();
    fn wait(&mut self, mutex: *mut core::ffi::c_void) -> core::ffi::c_int;
}
pub trait VtkCriticalSection {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn lock(&mut self) -> ();
    fn unlock(&mut self) -> ();
}
pub trait VtkDataArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn is_numeric(&mut self) -> core::ffi::c_int;
    fn get_element_component_size(&mut self) -> core::ffi::c_int;
    fn insert_tuple(
        &mut self,
        dstTupleIdx: core::ffi::c_longlong,
        srcTupleIdx: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_next_tuple(
        &mut self,
        srcTupleIdx: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong;
    fn insert_tuples(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn get_tuple_1(&mut self, tupleIdx: core::ffi::c_longlong) -> core::ffi::c_double;
    fn set_tuple(
        &mut self,
        dstTupleIdx: core::ffi::c_longlong,
        srcTupleIdx: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn set_tuple_1(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        value: core::ffi::c_double,
    ) -> ();
    fn set_tuple_2(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
    ) -> ();
    fn set_tuple_3(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
    ) -> ();
    fn set_tuple_4(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
    ) -> ();
    fn set_tuple_6(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
        val4: core::ffi::c_double,
        val5: core::ffi::c_double,
    ) -> ();
    fn set_tuple_9(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
        val4: core::ffi::c_double,
        val5: core::ffi::c_double,
        val6: core::ffi::c_double,
        val7: core::ffi::c_double,
        val8: core::ffi::c_double,
    ) -> ();
    fn insert_tuple_1(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        value: core::ffi::c_double,
    ) -> ();
    fn insert_tuple_2(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
    ) -> ();
    fn insert_tuple_3(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
    ) -> ();
    fn insert_tuple_4(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
    ) -> ();
    fn insert_tuple_6(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
        val4: core::ffi::c_double,
        val5: core::ffi::c_double,
    ) -> ();
    fn insert_tuple_9(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
        val4: core::ffi::c_double,
        val5: core::ffi::c_double,
        val6: core::ffi::c_double,
        val7: core::ffi::c_double,
        val8: core::ffi::c_double,
    ) -> ();
    fn insert_next_tuple_1(&mut self, value: core::ffi::c_double) -> ();
    fn insert_next_tuple_2(
        &mut self,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
    ) -> ();
    fn insert_next_tuple_3(
        &mut self,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
    ) -> ();
    fn insert_next_tuple_4(
        &mut self,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
    ) -> ();
    fn insert_next_tuple_6(
        &mut self,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
        val4: core::ffi::c_double,
        val5: core::ffi::c_double,
    ) -> ();
    fn insert_next_tuple_9(
        &mut self,
        val0: core::ffi::c_double,
        val1: core::ffi::c_double,
        val2: core::ffi::c_double,
        val3: core::ffi::c_double,
        val4: core::ffi::c_double,
        val5: core::ffi::c_double,
        val6: core::ffi::c_double,
        val7: core::ffi::c_double,
        val8: core::ffi::c_double,
    ) -> ();
    fn remove_tuple(&mut self, tupleIdx: core::ffi::c_longlong) -> ();
    fn remove_first_tuple(&mut self) -> ();
    fn remove_last_tuple(&mut self) -> ();
    fn get_component(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        compIdx: core::ffi::c_int,
    ) -> core::ffi::c_double;
    fn set_component(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        compIdx: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> ();
    fn insert_component(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        compIdx: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> ();
    fn get_data(
        &mut self,
        tupleMin: core::ffi::c_longlong,
        tupleMax: core::ffi::c_longlong,
        compMin: core::ffi::c_int,
        compMax: core::ffi::c_int,
        data: *mut core::ffi::c_void,
    ) -> ();
    fn deep_copy(&mut self, aa: *mut core::ffi::c_void) -> ();
    fn shallow_copy(&mut self, other: *mut core::ffi::c_void) -> ();
    fn fill_component(
        &mut self,
        compIdx: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> ();
    fn fill(&mut self, value: core::ffi::c_double) -> ();
    fn copy_component(
        &mut self,
        dstComponent: core::ffi::c_int,
        src: *mut core::ffi::c_void,
        srcComponent: core::ffi::c_int,
    ) -> ();
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong;
    fn create_default_lookup_table(&mut self) -> ();
    fn set_lookup_table(&mut self, lut: *mut core::ffi::c_void) -> ();
    fn get_lookup_table(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type_min(&mut self) -> core::ffi::c_double;
    fn get_data_type_max(&mut self) -> core::ffi::c_double;
    fn get_max_norm(&mut self) -> core::ffi::c_double;
    fn create_data_array(
        &mut self,
        dataType: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn component_range(&mut self) -> *mut core::ffi::c_void;
    fn l_2_norm_range(&mut self) -> *mut core::ffi::c_void;
    fn l_2_norm_finite_range(&mut self) -> *mut core::ffi::c_void;
    fn modified(&mut self) -> ();
    fn units_label(&mut self) -> *mut core::ffi::c_void;
    fn copy_information(
        &mut self,
        infoFrom: *mut core::ffi::c_void,
        deep: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_array_type(&mut self) -> core::ffi::c_int;
}
pub trait VtkDataArrayCollection {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn add_item(&mut self, ds: *mut core::ffi::c_void) -> ();
    fn get_next_item(&mut self) -> *mut core::ffi::c_void;
    fn get_item(&mut self, i: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn get_number_of_items(&mut self) -> core::ffi::c_int;
}
pub trait VtkDataArrayCollectionIterator {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_collection(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_data_array(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkDataArraySelection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn enable_array(&mut self, name: &str) -> ();
    fn disable_array(&mut self, name: &str) -> ();
    fn array_is_enabled(&mut self, name: &str) -> core::ffi::c_int;
    fn array_exists(&mut self, name: &str) -> core::ffi::c_int;
    fn enable_all_arrays(&mut self) -> ();
    fn disable_all_arrays(&mut self) -> ();
    fn get_number_of_arrays(&mut self) -> core::ffi::c_int;
    fn get_number_of_arrays_enabled(&mut self) -> core::ffi::c_int;
    fn get_array_name(&mut self, index: core::ffi::c_int) -> &str;
    fn get_array_index(&mut self, name: &str) -> core::ffi::c_int;
    fn get_enabled_array_index(&mut self, name: &str) -> core::ffi::c_int;
    fn get_array_setting(&mut self, index: core::ffi::c_int) -> core::ffi::c_int;
    fn set_array_setting(&mut self, name: &str, setting: core::ffi::c_int) -> ();
    fn remove_all_arrays(&mut self) -> ();
    fn add_array(&mut self, name: &str, state: bool) -> core::ffi::c_int;
    fn remove_array_by_index(&mut self, index: core::ffi::c_int) -> ();
    fn remove_array_by_name(&mut self, name: &str) -> ();
    fn copy_selections(&mut self, selections: *mut core::ffi::c_void) -> ();
    fn union(&mut self, other: *mut core::ffi::c_void) -> ();
    fn set_unknown_array_setting(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_unknown_array_setting(&mut self) -> core::ffi::c_int;
}
pub trait VtkDebugLeaks {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn construct_class(&mut self, object: *mut core::ffi::c_void) -> ();
    fn destruct_class(&mut self, object: *mut core::ffi::c_void) -> ();
    fn print_current_leaks(&mut self) -> core::ffi::c_int;
    fn get_exit_error(&mut self) -> core::ffi::c_int;
    fn set_exit_error(&mut self, p0: core::ffi::c_int) -> ();
    fn set_debug_leaks_observer(&mut self, observer: *mut core::ffi::c_void) -> ();
    fn get_debug_leaks_observer(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkDebugLeaksObserver {
    fn constructing_object(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn destructing_object(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkDenseArray {
    fn is_dense(&mut self) -> bool;
    fn get_non_null_size(&mut self) -> core::ffi::c_ulonglong;
    fn deep_copy(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkDoubleArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_double;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_double) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_double) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_double) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_double;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_double;
}
pub trait VtkDynamicLoader {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn lib_prefix(&mut self) -> &str;
    fn lib_extension(&mut self) -> &str;
    fn last_error(&mut self) -> &str;
}
pub trait VtkEventData {
    fn is_type_of(&mut self, type_: &str) -> core::ffi::c_int;
    fn is_a(&mut self, type_: &str) -> core::ffi::c_int;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_generations_from_base_type(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn get_number_of_generations_from_base(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn get_type(&mut self) -> core::ffi::c_int;
    fn set_type(&mut self, val: core::ffi::c_int) -> ();
    fn get_as_event_data_for_device(&mut self) -> *mut core::ffi::c_void;
    fn get_as_event_data_device_3_d(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkEventDataDevice3D {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_track_pad_position(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
    ) -> ();
}
pub trait VtkEventDataForDevice {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkEventForwarderCommand {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_target(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkFileOutputWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn display_text(&mut self, p0: &str) -> ();
    fn set_file_name(&mut self, _arg: &str) -> ();
    fn set_flush(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_flush(&mut self) -> core::ffi::c_int;
    fn flush_on(&mut self) -> ();
    fn flush_off(&mut self) -> ();
    fn set_append(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_append(&mut self) -> core::ffi::c_int;
    fn append_on(&mut self) -> ();
    fn append_off(&mut self) -> ();
}
pub trait VtkFloatArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_float;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_float) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_float) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_float) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_float;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_float;
}
pub trait VtkFloatingPointExceptions {
    fn enable(&mut self) -> ();
    fn disable(&mut self) -> ();
}
pub trait VtkGarbageCollector {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn collect(&mut self) -> ();
    fn deferred_collection_push(&mut self) -> ();
    fn deferred_collection_pop(&mut self) -> ();
    fn set_global_debug_flag(&mut self, flag: bool) -> ();
    fn get_global_debug_flag(&mut self) -> bool;
}
pub trait VtkGaussianRandomSequence {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_scaled_value(
        &mut self,
        mean: core::ffi::c_double,
        standardDeviation: core::ffi::c_double,
    ) -> core::ffi::c_double;
    fn get_next_scaled_value(
        &mut self,
        mean: core::ffi::c_double,
        standardDeviation: core::ffi::c_double,
    ) -> core::ffi::c_double;
}
pub trait VtkGenericDataArray {
    fn get_typed_tuple(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        tuple: *mut core::ffi::c_void,
    ) -> ();
    fn get_pointer(&mut self, valueIdx: core::ffi::c_longlong) -> *mut core::ffi::c_void;
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> ();
    fn write_pointer(
        &mut self,
        valueIdx: core::ffi::c_longlong,
        numValues: core::ffi::c_longlong,
    ) -> *mut core::ffi::c_void;
    fn remove_tuple(&mut self, tupleIdx: core::ffi::c_longlong) -> ();
    fn get_value_range(&mut self, comp: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn get_finite_value_range(
        &mut self,
        comp: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn capacity(&mut self) -> core::ffi::c_longlong;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn allocate(
        &mut self,
        size: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int;
    fn set_number_of_components(&mut self, num: core::ffi::c_int) -> ();
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> ();
    fn initialize(&mut self) -> ();
    fn squeeze(&mut self) -> ();
    fn clear_lookup(&mut self) -> ();
    fn data_changed(&mut self) -> ();
    fn new_iterator(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkGenericDataArrayLookupHelper {
    fn set_array(&mut self, array: *mut core::ffi::c_void) -> ();
    fn clear_lookup(&mut self) -> ();
}
pub trait VtkIdList {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn initialize(&mut self) -> ();
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        strategy: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_number_of_ids(&mut self) -> core::ffi::c_longlong;
    fn get_id(&mut self, i: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn find_id_location(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn set_number_of_ids(&mut self, number: core::ffi::c_longlong) -> ();
    fn set_id(&mut self, i: core::ffi::c_longlong, vtkid: core::ffi::c_longlong) -> ();
    fn insert_id(
        &mut self,
        i: core::ffi::c_longlong,
        vtkid: core::ffi::c_longlong,
    ) -> ();
    fn insert_next_id(&mut self, vtkid: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn insert_unique_id(
        &mut self,
        vtkid: core::ffi::c_longlong,
    ) -> core::ffi::c_longlong;
    fn sort(&mut self) -> ();
    fn fill(&mut self, value: core::ffi::c_longlong) -> ();
    fn reset(&mut self) -> ();
    fn squeeze(&mut self) -> ();
    fn deep_copy(&mut self, ids: *mut core::ffi::c_void) -> ();
    fn delete_id(&mut self, vtkid: core::ffi::c_longlong) -> ();
    fn is_id(&mut self, vtkid: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn intersect_with(&mut self, otherIds: *mut core::ffi::c_void) -> ();
}
pub trait VtkIdListCollection {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn add_item(&mut self, ds: *mut core::ffi::c_void) -> ();
    fn get_next_item(&mut self) -> *mut core::ffi::c_void;
    fn get_item(&mut self, i: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn get_number_of_items(&mut self) -> core::ffi::c_int;
}
pub trait VtkIdTypeArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn set_value(
        &mut self,
        id: core::ffi::c_longlong,
        value: core::ffi::c_longlong,
    ) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(
        &mut self,
        id: core::ffi::c_longlong,
        f: core::ffi::c_longlong,
    ) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_longlong;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_longlong;
}
pub trait VtkIndent {
    fn delete(&mut self) -> ();
    fn new(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkInformation {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn modified(&mut self) -> ();
    fn clear(&mut self) -> ();
    fn get_number_of_keys(&mut self) -> core::ffi::c_int;
    fn copy(&mut self, from: *mut core::ffi::c_void, deep: core::ffi::c_int) -> ();
    fn append(&mut self, from: *mut core::ffi::c_void, deep: core::ffi::c_int) -> ();
    fn copy_entry(
        &mut self,
        from: *mut core::ffi::c_void,
        key: *mut core::ffi::c_void,
        deep: core::ffi::c_int,
    ) -> ();
    fn copy_entries(
        &mut self,
        from: *mut core::ffi::c_void,
        key: *mut core::ffi::c_void,
        deep: core::ffi::c_int,
    ) -> ();
    fn has(&mut self, key: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn remove(&mut self, key: *mut core::ffi::c_void) -> ();
    fn set(&mut self, key: *mut core::ffi::c_void) -> ();
    fn get(&mut self, key: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn length(&mut self, key: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn append_unique(
        &mut self,
        key: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> ();
    fn get_key(&mut self, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn register(&mut self, o: *mut core::ffi::c_void) -> ();
    fn set_request(&mut self, request: *mut core::ffi::c_void) -> ();
    fn get_request(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkInformationDataObjectKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: *mut core::ffi::c_void) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationDoubleKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: core::ffi::c_double) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_double;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationDoubleVectorKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(
        &mut self,
        name: &str,
        location: &str,
        length: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn append(&mut self, info: *mut core::ffi::c_void, value: core::ffi::c_double) -> ();
    fn length(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationIdTypeKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: core::ffi::c_longlong) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_longlong;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationInformationKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: *mut core::ffi::c_void) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
    fn deep_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationInformationVectorKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: *mut core::ffi::c_void) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
    fn deep_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationIntegerKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: core::ffi::c_int) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationIntegerPointerKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn length(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationIntegerVectorKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(
        &mut self,
        name: &str,
        location: &str,
        length: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn append(&mut self, info: *mut core::ffi::c_void, value: core::ffi::c_int) -> ();
    fn length(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationIterator {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_information(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_information(&mut self) -> *mut core::ffi::c_void;
    fn set_information_weak(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn init_traversal(&mut self) -> ();
    fn go_to_first_item(&mut self) -> ();
    fn go_to_next_item(&mut self) -> ();
    fn is_done_with_traversal(&mut self) -> core::ffi::c_int;
    fn get_current_key(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkInformationKey {
    fn is_type_of(&mut self, type_: &str) -> core::ffi::c_int;
    fn is_a(&mut self, type_: &str) -> core::ffi::c_int;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_generations_from_base_type(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn get_number_of_generations_from_base(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn register(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn un_register(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_name(&mut self) -> &str;
    fn get_location(&mut self) -> &str;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
    fn deep_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
    fn has(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn remove(&mut self, info: *mut core::ffi::c_void) -> ();
    fn report(
        &mut self,
        info: *mut core::ffi::c_void,
        collector: *mut core::ffi::c_void,
    ) -> ();
    fn print(&mut self, info: *mut core::ffi::c_void) -> ();
    fn need_to_execute(
        &mut self,
        pipelineInfo: *mut core::ffi::c_void,
        dobjInfo: *mut core::ffi::c_void,
    ) -> bool;
    fn store_meta_data(
        &mut self,
        request: *mut core::ffi::c_void,
        pipelineInfo: *mut core::ffi::c_void,
        dobjInfo: *mut core::ffi::c_void,
    ) -> ();
    fn copy_default_information(
        &mut self,
        request: *mut core::ffi::c_void,
        fromInfo: *mut core::ffi::c_void,
        toInfo: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationKeyLookup {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn find(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
}
pub trait VtkInformationKeyVectorKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn append(
        &mut self,
        info: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> ();
    fn append_unique(
        &mut self,
        info: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> ();
    fn remove_item(
        &mut self,
        info: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> ();
    fn get(
        &mut self,
        info: *mut core::ffi::c_void,
        idx: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn length(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationObjectBaseKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(
        &mut self,
        name: &str,
        location: &str,
        requiredClass: &str,
    ) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: *mut core::ffi::c_void) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationObjectBaseVectorKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(
        &mut self,
        name: &str,
        location: &str,
        requiredClass: &str,
    ) -> *mut core::ffi::c_void;
    fn clear(&mut self, info: *mut core::ffi::c_void) -> ();
    fn resize(&mut self, info: *mut core::ffi::c_void, size: core::ffi::c_int) -> ();
    fn size(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn length(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn append(
        &mut self,
        info: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> ();
    fn set(
        &mut self,
        info: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        i: core::ffi::c_int,
    ) -> ();
    fn remove(
        &mut self,
        info: *mut core::ffi::c_void,
        val: *mut core::ffi::c_void,
    ) -> ();
    fn get(
        &mut self,
        info: *mut core::ffi::c_void,
        idx: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn shallow_copy(
        &mut self,
        source: *mut core::ffi::c_void,
        dest: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationRequestKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void) -> ();
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationStringKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: &str) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> &str;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationStringVectorKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(
        &mut self,
        name: &str,
        location: &str,
        length: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn append(&mut self, info: *mut core::ffi::c_void, value: &str) -> ();
    fn set(
        &mut self,
        info: *mut core::ffi::c_void,
        value: &str,
        index: core::ffi::c_int,
    ) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void, idx: core::ffi::c_int) -> &str;
    fn length(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationUnsignedLongKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(&mut self, info: *mut core::ffi::c_void, p1: core::ffi::c_ulong) -> ();
    fn get(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_ulong;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationVariantKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationVariantVectorKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(
        &mut self,
        name: &str,
        location: &str,
        length: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn length(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationVector {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_information_objects(&mut self) -> core::ffi::c_int;
    fn set_number_of_information_objects(&mut self, n: core::ffi::c_int) -> ();
    fn set_information_object(
        &mut self,
        index: core::ffi::c_int,
        info: *mut core::ffi::c_void,
    ) -> ();
    fn get_information_object(
        &mut self,
        index: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn append(&mut self, info: *mut core::ffi::c_void) -> ();
    fn remove(&mut self, info: *mut core::ffi::c_void) -> ();
    fn register(&mut self, o: *mut core::ffi::c_void) -> ();
    fn copy(&mut self, from: *mut core::ffi::c_void, deep: core::ffi::c_int) -> ();
}
pub trait VtkIntArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_int;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_int) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_int) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_int) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_int;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_int;
}
pub trait VtkLargeInteger {
    fn cast_to_char(&mut self) -> &str;
    fn cast_to_short(&mut self) -> core::ffi::c_short;
    fn cast_to_int(&mut self) -> core::ffi::c_int;
    fn cast_to_long(&mut self) -> core::ffi::c_long;
    fn cast_to_unsigned_long(&mut self) -> core::ffi::c_ulong;
    fn is_even(&mut self) -> core::ffi::c_int;
    fn is_odd(&mut self) -> core::ffi::c_int;
    fn get_length(&mut self) -> core::ffi::c_int;
    fn get_bit(&mut self, p: core::ffi::c_uint) -> core::ffi::c_int;
    fn is_zero(&mut self) -> core::ffi::c_int;
    fn get_sign(&mut self) -> core::ffi::c_int;
    fn truncate(&mut self, n: core::ffi::c_uint) -> ();
    fn complement(&mut self) -> ();
}
pub trait VtkLogger {
    fn is_type_of(&mut self, type_: &str) -> core::ffi::c_int;
    fn is_a(&mut self, type_: &str) -> core::ffi::c_int;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_generations_from_base_type(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn get_number_of_generations_from_base(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn init(&mut self) -> ();
    fn end_log_to_file(&mut self, path: &str) -> ();
    fn set_thread_name(&mut self, name: &str) -> ();
    fn remove_callback(&mut self, id: &str) -> bool;
    fn is_enabled(&mut self) -> bool;
    fn end_scope(&mut self, id: &str) -> ();
}
pub trait VtkLongArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_long;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_long) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_long) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_long) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_long;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_long;
}
pub trait VtkLongLongArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn set_value(
        &mut self,
        id: core::ffi::c_longlong,
        value: core::ffi::c_longlong,
    ) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(
        &mut self,
        id: core::ffi::c_longlong,
        f: core::ffi::c_longlong,
    ) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_longlong;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_longlong;
}
pub trait VtkLookupTable {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn is_opaque(&mut self) -> core::ffi::c_int;
    fn allocate(
        &mut self,
        sz: core::ffi::c_int,
        ext: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn build(&mut self) -> ();
    fn force_build(&mut self) -> ();
    fn build_special_colors(&mut self) -> ();
    fn set_ramp(&mut self, _arg: core::ffi::c_int) -> ();
    fn set_ramp_to_linear(&mut self) -> ();
    fn set_ramp_to_s_curve(&mut self) -> ();
    fn set_ramp_to_sqrt(&mut self) -> ();
    fn get_ramp(&mut self) -> core::ffi::c_int;
    fn set_scale(&mut self, scale: core::ffi::c_int) -> ();
    fn set_scale_to_linear(&mut self) -> ();
    fn set_scale_to_log_10(&mut self) -> ();
    fn get_scale(&mut self) -> core::ffi::c_int;
    fn set_table_range(
        &mut self,
        min: core::ffi::c_double,
        max: core::ffi::c_double,
    ) -> ();
    fn set_hue_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> ();
    fn set_saturation_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> ();
    fn set_value_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> ();
    fn set_alpha_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> ();
    fn set_nan_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
    ) -> ();
    fn set_below_range_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
    ) -> ();
    fn set_use_below_range_color(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_use_below_range_color(&mut self) -> core::ffi::c_int;
    fn use_below_range_color_on(&mut self) -> ();
    fn use_below_range_color_off(&mut self) -> ();
    fn set_above_range_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
    ) -> ();
    fn set_use_above_range_color(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_use_above_range_color(&mut self) -> core::ffi::c_int;
    fn use_above_range_color_on(&mut self) -> ();
    fn use_above_range_color_off(&mut self) -> ();
    fn get_opacity(&mut self, v: core::ffi::c_double) -> core::ffi::c_double;
    fn get_index(&mut self, v: core::ffi::c_double) -> core::ffi::c_longlong;
    fn set_number_of_table_values(&mut self, number: core::ffi::c_longlong) -> ();
    fn get_number_of_table_values(&mut self) -> core::ffi::c_longlong;
    fn set_table_value(
        &mut self,
        indx: core::ffi::c_longlong,
        r: core::ffi::c_double,
        g: core::ffi::c_double,
        b: core::ffi::c_double,
        a: core::ffi::c_double,
    ) -> ();
    fn set_number_of_colors(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_number_of_colors_min_value(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_colors_max_value(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_colors(&mut self) -> core::ffi::c_longlong;
    fn set_table(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_table(&mut self) -> *mut core::ffi::c_void;
    fn deep_copy(&mut self, obj: *mut core::ffi::c_void) -> ();
    fn using_log_scale(&mut self) -> core::ffi::c_int;
}
pub trait VtkMappedDataArray {
    fn deep_copy(&mut self, aa: *mut core::ffi::c_void) -> ();
    fn get_tuples(
        &mut self,
        ptIds: *mut core::ffi::c_void,
        output: *mut core::ffi::c_void,
    ) -> ();
    fn data_changed(&mut self) -> ();
    fn modified(&mut self) -> ();
}
pub trait VtkMath {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn pi(&mut self) -> core::ffi::c_double;
    fn radians_from_degrees(
        &mut self,
        degrees: core::ffi::c_float,
    ) -> core::ffi::c_float;
    fn degrees_from_radians(
        &mut self,
        radians: core::ffi::c_float,
    ) -> core::ffi::c_float;
    fn round(&mut self, f: core::ffi::c_float) -> core::ffi::c_int;
    fn floor(&mut self, x: core::ffi::c_double) -> core::ffi::c_int;
    fn ceil(&mut self, x: core::ffi::c_double) -> core::ffi::c_int;
    fn ceil_log_2(&mut self, x: core::ffi::c_ulonglong) -> core::ffi::c_int;
    fn is_power_of_two(&mut self, x: core::ffi::c_ulonglong) -> bool;
    fn nearest_power_of_two(&mut self, x: core::ffi::c_int) -> core::ffi::c_int;
    fn factorial(&mut self, N: core::ffi::c_int) -> core::ffi::c_longlong;
    fn binomial(
        &mut self,
        m: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> core::ffi::c_longlong;
    fn random_seed(&mut self, s: core::ffi::c_int) -> ();
    fn get_seed(&mut self) -> core::ffi::c_int;
    fn random(&mut self) -> core::ffi::c_double;
    fn gaussian(&mut self) -> core::ffi::c_double;
    fn gaussian_amplitude(
        &mut self,
        variance: core::ffi::c_double,
        distanceFromMean: core::ffi::c_double,
    ) -> core::ffi::c_double;
    fn gaussian_weight(
        &mut self,
        variance: core::ffi::c_double,
        distanceFromMean: core::ffi::c_double,
    ) -> core::ffi::c_double;
    fn determinant_2_x_2(
        &mut self,
        a: core::ffi::c_double,
        b: core::ffi::c_double,
        c: core::ffi::c_double,
        d: core::ffi::c_double,
    ) -> core::ffi::c_double;
    fn determinant_3_x_3(
        &mut self,
        a1: core::ffi::c_double,
        a2: core::ffi::c_double,
        a3: core::ffi::c_double,
        b1: core::ffi::c_double,
        b2: core::ffi::c_double,
        b3: core::ffi::c_double,
        c1: core::ffi::c_double,
        c2: core::ffi::c_double,
        c3: core::ffi::c_double,
    ) -> core::ffi::c_double;
    fn solve_linear_system_gepp_2_x_2(
        &mut self,
        a00: core::ffi::c_double,
        a01: core::ffi::c_double,
        a10: core::ffi::c_double,
        a11: core::ffi::c_double,
        b0: core::ffi::c_double,
        b1: core::ffi::c_double,
        x0: &mut core::ffi::c_double,
        x1: &mut core::ffi::c_double,
    ) -> core::ffi::c_int;
    fn get_scalar_type_fitting_range(
        &mut self,
        range_min: core::ffi::c_double,
        range_max: core::ffi::c_double,
        scale: core::ffi::c_double,
        shift: core::ffi::c_double,
    ) -> core::ffi::c_int;
    fn inf(&mut self) -> core::ffi::c_double;
    fn neg_inf(&mut self) -> core::ffi::c_double;
    fn nan(&mut self) -> core::ffi::c_double;
    fn is_inf(&mut self, x: core::ffi::c_double) -> core::ffi::c_int;
    fn is_nan(&mut self, x: core::ffi::c_double) -> core::ffi::c_int;
    fn is_finite(&mut self, x: core::ffi::c_double) -> bool;
}
pub trait VtkMersenneTwister {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn initialize(&mut self, seed: core::ffi::c_uint) -> ();
    fn initialize_new_sequence(
        &mut self,
        seed: core::ffi::c_uint,
        p: core::ffi::c_int,
    ) -> core::ffi::c_uint;
    fn initialize_sequence(
        &mut self,
        id: core::ffi::c_uint,
        seed: core::ffi::c_uint,
        p: core::ffi::c_int,
    ) -> ();
    fn get_value(&mut self, id: core::ffi::c_uint) -> core::ffi::c_double;
    fn next(&mut self, id: core::ffi::c_uint) -> ();
}
pub trait VtkMinimalStandardRandomSequence {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn initialize(&mut self, seed: core::ffi::c_uint) -> ();
    fn set_seed(&mut self, value: core::ffi::c_int) -> ();
    fn set_seed_only(&mut self, value: core::ffi::c_int) -> ();
    fn get_seed(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self) -> core::ffi::c_double;
    fn next(&mut self) -> ();
    fn get_range_value(
        &mut self,
        rangeMin: core::ffi::c_double,
        rangeMax: core::ffi::c_double,
    ) -> core::ffi::c_double;
    fn get_next_range_value(
        &mut self,
        rangeMin: core::ffi::c_double,
        rangeMax: core::ffi::c_double,
    ) -> core::ffi::c_double;
}
pub trait VtkMultiThreader {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_number_of_threads(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_threads_min_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_threads_max_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_threads(&mut self) -> core::ffi::c_int;
    fn get_global_static_maximum_number_of_threads(&mut self) -> core::ffi::c_int;
    fn set_global_maximum_number_of_threads(&mut self, val: core::ffi::c_int) -> ();
    fn get_global_maximum_number_of_threads(&mut self) -> core::ffi::c_int;
    fn set_global_default_number_of_threads(&mut self, val: core::ffi::c_int) -> ();
    fn get_global_default_number_of_threads(&mut self) -> core::ffi::c_int;
    fn single_method_execute(&mut self) -> ();
    fn multiple_method_execute(&mut self) -> ();
    fn terminate_thread(&mut self, threadId: core::ffi::c_int) -> ();
    fn is_thread_active(&mut self, threadId: core::ffi::c_int) -> core::ffi::c_int;
}
pub trait VtkMutexLock {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn lock(&mut self) -> ();
    fn unlock(&mut self) -> ();
}
pub trait VtkNew {
    fn reset(&mut self) -> ();
    fn get_pointer(&mut self) -> *mut core::ffi::c_void;
    fn get(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkOStrStreamWrapper {
    fn rdbuf(&mut self) -> *mut core::ffi::c_void;
    fn freeze(&mut self) -> ();
}
pub trait VtkObject {
    fn is_type_of(&mut self, type_: &str) -> core::ffi::c_int;
    fn is_a(&mut self, type_: &str) -> core::ffi::c_int;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_generations_from_base_type(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn get_number_of_generations_from_base(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn debug_on(&mut self) -> ();
    fn debug_off(&mut self) -> ();
    fn get_debug(&mut self) -> bool;
    fn set_debug(&mut self, debugFlag: bool) -> ();
    fn break_on_error(&mut self) -> ();
    fn modified(&mut self) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn set_global_warning_display(&mut self, val: core::ffi::c_int) -> ();
    fn global_warning_display_on(&mut self) -> ();
    fn global_warning_display_off(&mut self) -> ();
    fn get_global_warning_display(&mut self) -> core::ffi::c_int;
    fn add_observer(
        &mut self,
        event: core::ffi::c_ulong,
        p1: *mut core::ffi::c_void,
        priority: core::ffi::c_float,
    ) -> core::ffi::c_ulong;
    fn get_command(&mut self, tag: core::ffi::c_ulong) -> *mut core::ffi::c_void;
    fn remove_observer(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn remove_observers(
        &mut self,
        event: core::ffi::c_ulong,
        p1: *mut core::ffi::c_void,
    ) -> ();
    fn has_observer(
        &mut self,
        event: core::ffi::c_ulong,
        p1: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn remove_all_observers(&mut self) -> ();
}
pub trait VtkObjectBase {
    fn get_class_name(&mut self) -> &str;
    fn is_type_of(&mut self, name: &str) -> core::ffi::c_int;
    fn is_a(&mut self, name: &str) -> core::ffi::c_int;
    fn get_number_of_generations_from_base_type(
        &mut self,
        name: &str,
    ) -> core::ffi::c_longlong;
    fn get_number_of_generations_from_base(
        &mut self,
        name: &str,
    ) -> core::ffi::c_longlong;
    fn delete(&mut self) -> ();
    fn fast_delete(&mut self) -> ();
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn initialize_object_base(&mut self) -> ();
    fn register(&mut self, o: *mut core::ffi::c_void) -> ();
    fn un_register(&mut self, o: *mut core::ffi::c_void) -> ();
    fn get_reference_count(&mut self) -> core::ffi::c_int;
    fn set_reference_count(&mut self, p0: core::ffi::c_int) -> ();
    fn set_memkind_directory(&mut self, directoryname: &str) -> ();
    fn get_using_memkind(&mut self) -> bool;
    fn get_is_in_memkind(&mut self) -> bool;
}
pub trait VtkObjectFactory {
    fn create_instance(
        &mut self,
        vtkclassname: &str,
        isAbstract: bool,
    ) -> *mut core::ffi::c_void;
    fn create_all_instance(
        &mut self,
        vtkclassname: &str,
        retList: *mut core::ffi::c_void,
    ) -> ();
    fn re_hash(&mut self) -> ();
    fn register_factory(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn un_register_factory(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn un_register_all_factories(&mut self) -> ();
    fn get_registered_factories(&mut self) -> *mut core::ffi::c_void;
    fn has_override_any(&mut self, className: &str) -> core::ffi::c_int;
    fn get_override_information(&mut self, name: &str, p1: *mut core::ffi::c_void) -> ();
    fn set_all_enable_flags(&mut self, flag: core::ffi::c_int, className: &str) -> ();
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_vtk_source_version(&mut self) -> &str;
    fn get_description(&mut self) -> &str;
    fn get_number_of_overrides(&mut self) -> core::ffi::c_int;
    fn get_class_override_name(&mut self, index: core::ffi::c_int) -> &str;
    fn get_class_override_with_name(&mut self, index: core::ffi::c_int) -> &str;
    fn get_enable_flag(&mut self, index: core::ffi::c_int) -> core::ffi::c_int;
    fn get_override_description(&mut self, index: core::ffi::c_int) -> &str;
    fn set_enable_flag(
        &mut self,
        flag: core::ffi::c_int,
        className: &str,
        subclassName: &str,
    ) -> ();
    fn has_override(&mut self, className: &str) -> core::ffi::c_int;
    fn disable(&mut self, className: &str) -> ();
}
pub trait VtkObjectFactoryCollection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn add_item(&mut self, t: *mut core::ffi::c_void) -> ();
    fn get_next_item(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkOldStyleCallbackCommand {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_callback(&mut self, f: *mut core::ffi::c_void) -> ();
    fn set_client_data_delete_callback(&mut self, f: *mut core::ffi::c_void) -> ();
}
pub trait VtkOutputWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_instance(&mut self, instance: *mut core::ffi::c_void) -> ();
    fn display_text(&mut self, p0: &str) -> ();
    fn display_error_text(&mut self, p0: &str) -> ();
    fn display_warning_text(&mut self, p0: &str) -> ();
    fn display_generic_warning_text(&mut self, p0: &str) -> ();
    fn display_debug_text(&mut self, p0: &str) -> ();
    fn prompt_user_on(&mut self) -> ();
    fn prompt_user_off(&mut self) -> ();
    fn set_prompt_user(&mut self, _arg: bool) -> ();
    fn set_use_std_error_for_all_messages(&mut self, p0: bool) -> ();
    fn get_use_std_error_for_all_messages(&mut self) -> bool;
    fn use_std_error_for_all_messages_on(&mut self) -> ();
    fn use_std_error_for_all_messages_off(&mut self) -> ();
    fn set_display_mode(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_display_mode_min_value(&mut self) -> core::ffi::c_int;
    fn get_display_mode_max_value(&mut self) -> core::ffi::c_int;
    fn get_display_mode(&mut self) -> core::ffi::c_int;
    fn set_display_mode_to_default(&mut self) -> ();
    fn set_display_mode_to_never(&mut self) -> ();
    fn set_display_mode_to_always(&mut self) -> ();
    fn set_display_mode_to_always_std_err(&mut self) -> ();
}
pub trait VtkOverrideInformation {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_class_override_name(&mut self) -> &str;
    fn get_class_override_with_name(&mut self) -> &str;
    fn get_description(&mut self) -> &str;
    fn get_object_factory(&mut self) -> *mut core::ffi::c_void;
    fn set_class_override_name(&mut self, _arg: &str) -> ();
    fn set_class_override_with_name(&mut self, _arg: &str) -> ();
    fn set_description(&mut self, _arg: &str) -> ();
}
pub trait VtkOverrideInformationCollection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn add_item(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_next_item(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkPoints {
    fn new(&mut self, dataType: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn set_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_data(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn set_data_type(&mut self, dataType: core::ffi::c_int) -> ();
    fn set_data_type_to_bit(&mut self) -> ();
    fn set_data_type_to_char(&mut self) -> ();
    fn set_data_type_to_unsigned_char(&mut self) -> ();
    fn set_data_type_to_short(&mut self) -> ();
    fn set_data_type_to_unsigned_short(&mut self) -> ();
    fn set_data_type_to_int(&mut self) -> ();
    fn set_data_type_to_unsigned_int(&mut self) -> ();
    fn set_data_type_to_long(&mut self) -> ();
    fn set_data_type_to_unsigned_long(&mut self) -> ();
    fn set_data_type_to_float(&mut self) -> ();
    fn set_data_type_to_double(&mut self) -> ();
    fn squeeze(&mut self) -> ();
    fn reset(&mut self) -> ();
    fn deep_copy(&mut self, ad: *mut core::ffi::c_void) -> ();
    fn shallow_copy(&mut self, ad: *mut core::ffi::c_void) -> ();
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong;
    fn get_number_of_points(&mut self) -> core::ffi::c_longlong;
    fn set_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn insert_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn insert_points(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_next_point(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> core::ffi::c_longlong;
    fn set_number_of_points(&mut self, numPoints: core::ffi::c_longlong) -> ();
    fn resize(&mut self, numPoints: core::ffi::c_longlong) -> core::ffi::c_int;
    fn get_points(
        &mut self,
        ptId: *mut core::ffi::c_void,
        outPoints: *mut core::ffi::c_void,
    ) -> ();
    fn compute_bounds(&mut self) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn modified(&mut self) -> ();
}
pub trait VtkPoints2D {
    fn new(&mut self, dataType: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn set_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_data(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn set_data_type(&mut self, dataType: core::ffi::c_int) -> ();
    fn set_data_type_to_bit(&mut self) -> ();
    fn set_data_type_to_char(&mut self) -> ();
    fn set_data_type_to_unsigned_char(&mut self) -> ();
    fn set_data_type_to_short(&mut self) -> ();
    fn set_data_type_to_unsigned_short(&mut self) -> ();
    fn set_data_type_to_int(&mut self) -> ();
    fn set_data_type_to_unsigned_int(&mut self) -> ();
    fn set_data_type_to_long(&mut self) -> ();
    fn set_data_type_to_unsigned_long(&mut self) -> ();
    fn set_data_type_to_float(&mut self) -> ();
    fn set_data_type_to_double(&mut self) -> ();
    fn squeeze(&mut self) -> ();
    fn reset(&mut self) -> ();
    fn deep_copy(&mut self, ad: *mut core::ffi::c_void) -> ();
    fn shallow_copy(&mut self, ad: *mut core::ffi::c_void) -> ();
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong;
    fn get_number_of_points(&mut self) -> core::ffi::c_longlong;
    fn set_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
    ) -> ();
    fn insert_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
    ) -> ();
    fn insert_next_point(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
    ) -> core::ffi::c_longlong;
    fn remove_point(&mut self, id: core::ffi::c_longlong) -> ();
    fn set_number_of_points(&mut self, numPoints: core::ffi::c_longlong) -> ();
    fn resize(&mut self, numPoints: core::ffi::c_longlong) -> core::ffi::c_int;
    fn get_points(
        &mut self,
        ptId: *mut core::ffi::c_void,
        fp: *mut core::ffi::c_void,
    ) -> ();
    fn compute_bounds(&mut self) -> ();
}
pub trait VtkPriorityQueue {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn allocate(&mut self, sz: core::ffi::c_longlong, ext: core::ffi::c_longlong) -> ();
    fn insert(&mut self, priority: core::ffi::c_double, id: core::ffi::c_longlong) -> ();
    fn pop(
        &mut self,
        location: core::ffi::c_longlong,
        priority: &mut core::ffi::c_double,
    ) -> core::ffi::c_longlong;
    fn peek(
        &mut self,
        location: core::ffi::c_longlong,
        priority: &mut core::ffi::c_double,
    ) -> core::ffi::c_longlong;
    fn delete_id(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_double;
    fn get_priority(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_double;
    fn get_number_of_items(&mut self) -> core::ffi::c_longlong;
    fn reset(&mut self) -> ();
}
pub trait VtkRandomPool {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_sequence(&mut self, seq: *mut core::ffi::c_void) -> ();
    fn get_sequence(&mut self) -> *mut core::ffi::c_void;
    fn set_size(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_size_min_value(&mut self) -> core::ffi::c_longlong;
    fn get_size_max_value(&mut self) -> core::ffi::c_longlong;
    fn get_size(&mut self) -> core::ffi::c_longlong;
    fn set_number_of_components(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_number_of_components_min_value(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_components_max_value(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_components(&mut self) -> core::ffi::c_longlong;
    fn get_total_size(&mut self) -> core::ffi::c_longlong;
    fn get_value(&mut self, i: core::ffi::c_longlong) -> core::ffi::c_double;
    fn populate_data_array(
        &mut self,
        da: *mut core::ffi::c_void,
        minRange: core::ffi::c_double,
        maxRange: core::ffi::c_double,
    ) -> ();
    fn set_chunk_size(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_chunk_size_min_value(&mut self) -> core::ffi::c_longlong;
    fn get_chunk_size_max_value(&mut self) -> core::ffi::c_longlong;
    fn get_chunk_size(&mut self) -> core::ffi::c_longlong;
}
pub trait VtkRandomSequence {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn initialize(&mut self, seed: core::ffi::c_uint) -> ();
    fn get_value(&mut self) -> core::ffi::c_double;
    fn next(&mut self) -> ();
    fn get_next_value(&mut self) -> core::ffi::c_double;
}
pub trait VtkReferenceCount {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkSMPThreadLocal {
    fn size(&mut self) -> usize;
}
pub trait VtkSMPThreadLocalObject {
    fn local(&mut self) -> *mut core::ffi::c_void;
    fn size(&mut self) -> usize;
}
pub trait VtkSMPTools {
    fn get_backend(&mut self) -> &str;
    fn set_backend(&mut self, backend: &str) -> bool;
    fn initialize(&mut self, numThreads: core::ffi::c_int) -> ();
    fn get_estimated_number_of_threads(&mut self) -> core::ffi::c_int;
    fn set_nested_parallelism(&mut self, isNested: bool) -> ();
    fn get_nested_parallelism(&mut self) -> bool;
    fn is_parallel_scope(&mut self) -> bool;
}
pub trait VtkSOADataArrayTemplate {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_typed_tuple(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        tuple: *mut core::ffi::c_void,
    ) -> ();
    fn set_array(
        &mut self,
        comp: core::ffi::c_int,
        array: *mut core::ffi::c_void,
        size: core::ffi::c_longlong,
        updateMaxId: bool,
        save: bool,
        deleteMethod: core::ffi::c_int,
    ) -> ();
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> ();
    fn get_component_array_pointer(
        &mut self,
        comp: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_number_of_components(&mut self, numComps: core::ffi::c_int) -> ();
}
pub trait VtkScalarsToColors {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn is_opaque(&mut self) -> core::ffi::c_int;
    fn build(&mut self) -> ();
    fn set_range(&mut self, min: core::ffi::c_double, max: core::ffi::c_double) -> ();
    fn get_opacity(&mut self, v: core::ffi::c_double) -> core::ffi::c_double;
    fn get_luminance(&mut self, x: core::ffi::c_double) -> core::ffi::c_double;
    fn set_alpha(&mut self, alpha: core::ffi::c_double) -> ();
    fn get_alpha(&mut self) -> core::ffi::c_double;
    fn map_scalars(
        &mut self,
        scalars: *mut core::ffi::c_void,
        colorMode: core::ffi::c_int,
        component: core::ffi::c_int,
        outputFormat: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_vector_mode(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_vector_mode(&mut self) -> core::ffi::c_int;
    fn set_vector_mode_to_magnitude(&mut self) -> ();
    fn set_vector_mode_to_component(&mut self) -> ();
    fn set_vector_mode_to_rgb_colors(&mut self) -> ();
    fn set_vector_component(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_vector_component(&mut self) -> core::ffi::c_int;
    fn set_vector_size(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_vector_size(&mut self) -> core::ffi::c_int;
    fn deep_copy(&mut self, o: *mut core::ffi::c_void) -> ();
    fn using_log_scale(&mut self) -> core::ffi::c_int;
    fn get_number_of_available_colors(&mut self) -> core::ffi::c_longlong;
    fn set_annotations(
        &mut self,
        values: *mut core::ffi::c_void,
        annotations: *mut core::ffi::c_void,
    ) -> ();
    fn get_annotated_values(&mut self) -> *mut core::ffi::c_void;
    fn get_annotations(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_annotated_values(&mut self) -> core::ffi::c_longlong;
    fn reset_annotations(&mut self) -> ();
    fn set_indexed_lookup(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_indexed_lookup(&mut self) -> core::ffi::c_int;
    fn indexed_lookup_on(&mut self) -> ();
    fn indexed_lookup_off(&mut self) -> ();
}
pub trait VtkShortArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_short;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_short) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_short) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_short) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_short;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_short;
}
pub trait VtkSignedCharArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_schar;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_schar) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_schar) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_schar) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_schar;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_schar;
}
pub trait VtkSimpleConditionVariable {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn delete(&mut self) -> ();
    fn signal(&mut self) -> ();
    fn broadcast(&mut self) -> ();
}
pub trait VtkSimpleCriticalSection {
    fn init(&mut self) -> ();
    fn lock(&mut self) -> ();
    fn unlock(&mut self) -> ();
}
pub trait VtkSimpleMutexLock {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn delete(&mut self) -> ();
    fn lock(&mut self) -> ();
    fn unlock(&mut self) -> ();
}
pub trait VtkSmartPointer {
    fn get_pointer(&mut self) -> *mut core::ffi::c_void;
    fn get(&mut self) -> *mut core::ffi::c_void;
    fn take_reference(&mut self, t: *mut core::ffi::c_void) -> ();
}
pub trait VtkSmartPointerBase {
    fn get_pointer(&mut self) -> *mut core::ffi::c_void;
    fn report(&mut self, collector: *mut core::ffi::c_void, desc: &str) -> ();
}
pub trait VtkSortDataArray {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn sort(&mut self, keys: *mut core::ffi::c_void) -> ();
    fn sort_array_by_component(
        &mut self,
        arr: *mut core::ffi::c_void,
        k: core::ffi::c_int,
    ) -> ();
}
pub trait VtkSparseArray {
    fn is_dense(&mut self) -> bool;
    fn get_non_null_size(&mut self) -> core::ffi::c_ulonglong;
    fn deep_copy(&mut self) -> *mut core::ffi::c_void;
    fn clear(&mut self) -> ();
    fn reserve_storage(&mut self, value_count: core::ffi::c_ulonglong) -> ();
    fn set_extents_from_contents(&mut self) -> ();
    fn validate(&mut self) -> bool;
}
pub trait VtkStringArray {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn is_numeric(&mut self) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn squeeze(&mut self) -> ();
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int;
    fn set_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuples(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_next_tuple(
        &mut self,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong;
    fn get_tuples(
        &mut self,
        ptIds: *mut core::ffi::c_void,
        output: *mut core::ffi::c_void,
    ) -> ();
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> ();
    fn get_number_of_values(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_element_components(&mut self) -> core::ffi::c_int;
    fn get_element_component_size(&mut self) -> core::ffi::c_int;
    fn write_pointer(
        &mut self,
        id: core::ffi::c_longlong,
        number: core::ffi::c_longlong,
    ) -> *mut core::ffi::c_void;
    fn get_pointer(&mut self, id: core::ffi::c_longlong) -> *mut core::ffi::c_void;
    fn deep_copy(&mut self, aa: *mut core::ffi::c_void) -> ();
    fn set_array(
        &mut self,
        array: *mut core::ffi::c_void,
        size: core::ffi::c_longlong,
        save: core::ffi::c_int,
        deleteMethod: core::ffi::c_int,
    ) -> ();
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> ();
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong;
    fn new_iterator(&mut self) -> *mut core::ffi::c_void;
    fn get_data_size(&mut self) -> core::ffi::c_longlong;
    fn data_changed(&mut self) -> ();
    fn data_element_changed(&mut self, id: core::ffi::c_longlong) -> ();
    fn clear_lookup(&mut self) -> ();
}
pub trait VtkStringOutputWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn display_text(&mut self, p0: &str) -> ();
}
pub trait VtkTestDataArray {
    fn get_typed_tuple(
        &mut self,
        tupleIdx: core::ffi::c_longlong,
        tuple: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkTimePointUtility {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn date_to_time_point(
        &mut self,
        year: core::ffi::c_int,
        month: core::ffi::c_int,
        day: core::ffi::c_int,
    ) -> core::ffi::c_ulonglong;
    fn time_to_time_point(
        &mut self,
        hour: core::ffi::c_int,
        minute: core::ffi::c_int,
        second: core::ffi::c_int,
        millis: core::ffi::c_int,
    ) -> core::ffi::c_ulonglong;
    fn date_time_to_time_point(
        &mut self,
        year: core::ffi::c_int,
        month: core::ffi::c_int,
        day: core::ffi::c_int,
        hour: core::ffi::c_int,
        minute: core::ffi::c_int,
        sec: core::ffi::c_int,
        millis: core::ffi::c_int,
    ) -> core::ffi::c_ulonglong;
    fn get_date(
        &mut self,
        time: core::ffi::c_ulonglong,
        year: &mut core::ffi::c_int,
        month: &mut core::ffi::c_int,
        day: &mut core::ffi::c_int,
    ) -> ();
    fn get_time(
        &mut self,
        time: core::ffi::c_ulonglong,
        hour: &mut core::ffi::c_int,
        minute: &mut core::ffi::c_int,
        second: &mut core::ffi::c_int,
        millis: &mut core::ffi::c_int,
    ) -> ();
    fn get_date_time(
        &mut self,
        time: core::ffi::c_ulonglong,
        year: &mut core::ffi::c_int,
        month: &mut core::ffi::c_int,
        day: &mut core::ffi::c_int,
        hour: &mut core::ffi::c_int,
        minute: &mut core::ffi::c_int,
        second: &mut core::ffi::c_int,
        millis: &mut core::ffi::c_int,
    ) -> ();
    fn get_year(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int;
    fn get_month(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int;
    fn get_day(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int;
    fn get_hour(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int;
    fn get_minute(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int;
    fn get_second(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int;
    fn get_millisecond(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int;
    fn time_point_to_iso_8601(
        &mut self,
        p0: core::ffi::c_ulonglong,
        format: core::ffi::c_int,
    ) -> &str;
}
pub trait VtkTimeStamp {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn delete(&mut self) -> ();
    fn modified(&mut self) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkTypeFloat32Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeFloat64Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeInt16Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeInt32Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeInt64Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeInt8Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeUInt16Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeUInt32Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeUInt64Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypeUInt8Array {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
pub trait VtkTypedArray {}
pub trait VtkTypedDataArray {
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn get_typed_tuple(
        &mut self,
        idx: core::ffi::c_longlong,
        t: *mut core::ffi::c_void,
    ) -> ();
    fn allocate(
        &mut self,
        size: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
}
pub trait VtkUnicodeString {
    fn is_utf_8(&mut self, p0: &str) -> bool;
    fn utf_8_str(&mut self) -> &str;
    fn empty(&mut self) -> bool;
    fn push_back(&mut self, p0: core::ffi::c_uint) -> ();
    fn clear(&mut self) -> ();
}
pub trait VtkUnicodeStringArray {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn get_element_component_size(&mut self) -> core::ffi::c_int;
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> ();
    fn set_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuples(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_next_tuple(
        &mut self,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong;
    fn squeeze(&mut self) -> ();
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int;
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> ();
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong;
    fn is_numeric(&mut self) -> core::ffi::c_int;
    fn new_iterator(&mut self) -> *mut core::ffi::c_void;
    fn data_changed(&mut self) -> ();
    fn clear_lookup(&mut self) -> ();
    fn insert_next_utf_8_value(&mut self, p0: &str) -> ();
    fn set_utf_8_value(&mut self, i: core::ffi::c_longlong, p1: &str) -> ();
    fn get_utf_8_value(&mut self, i: core::ffi::c_longlong) -> &str;
}
pub trait VtkUnsignedCharArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_uchar;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_uchar) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_uchar) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_uchar) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_uchar;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_uchar;
}
pub trait VtkUnsignedIntArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_uint;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_uint) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_uint) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_uint) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_uint;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_uint;
}
pub trait VtkUnsignedLongArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_ulong;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_ulong) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_ulong) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_ulong) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_ulong;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkUnsignedLongLongArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_ulonglong;
    fn set_value(
        &mut self,
        id: core::ffi::c_longlong,
        value: core::ffi::c_ulonglong,
    ) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(
        &mut self,
        id: core::ffi::c_longlong,
        f: core::ffi::c_ulonglong,
    ) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_ulonglong) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_ulonglong;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_ulonglong;
}
pub trait VtkUnsignedShortArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_ushort;
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_ushort) -> ();
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool;
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_ushort) -> ();
    fn insert_next_value(&mut self, f: core::ffi::c_ushort) -> core::ffi::c_longlong;
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_data_type_value_min(&mut self) -> core::ffi::c_ushort;
    fn get_data_type_value_max(&mut self) -> core::ffi::c_ushort;
}
pub trait VtkVariant {
    fn is_valid(&mut self) -> bool;
    fn is_string(&mut self) -> bool;
    fn is_unicode_string(&mut self) -> bool;
    fn is_numeric(&mut self) -> bool;
    fn is_float(&mut self) -> bool;
    fn is_double(&mut self) -> bool;
    fn is_char(&mut self) -> bool;
    fn is_unsigned_char(&mut self) -> bool;
    fn is_signed_char(&mut self) -> bool;
    fn is_short(&mut self) -> bool;
    fn is_unsigned_short(&mut self) -> bool;
    fn is_int(&mut self) -> bool;
    fn is_unsigned_int(&mut self) -> bool;
    fn is_long(&mut self) -> bool;
    fn is_unsigned_long(&mut self) -> bool;
    fn is__int_64(&mut self) -> bool;
    fn is_unsigned__int_64(&mut self) -> bool;
    fn is_long_long(&mut self) -> bool;
    fn is_unsigned_long_long(&mut self) -> bool;
    fn is_vtk_object(&mut self) -> bool;
    fn is_array(&mut self) -> bool;
    fn get_type(&mut self) -> core::ffi::c_uint;
    fn get_type_as_string(&mut self) -> &str;
    fn to_vtk_object(&mut self) -> *mut core::ffi::c_void;
    fn to_array(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkVariantArray {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn get_element_component_size(&mut self) -> core::ffi::c_int;
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> ();
    fn set_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_tuples(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> ();
    fn insert_next_tuple(
        &mut self,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong;
    fn deep_copy(&mut self, da: *mut core::ffi::c_void) -> ();
    fn squeeze(&mut self) -> ();
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int;
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong;
    fn is_numeric(&mut self) -> core::ffi::c_int;
    fn new_iterator(&mut self) -> *mut core::ffi::c_void;
    fn get_pointer(&mut self, id: core::ffi::c_longlong) -> *mut core::ffi::c_void;
    fn set_array(
        &mut self,
        arr: *mut core::ffi::c_void,
        size: core::ffi::c_longlong,
        save: core::ffi::c_int,
        deleteMethod: core::ffi::c_int,
    ) -> ();
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> ();
    fn get_number_of_values(&mut self) -> core::ffi::c_longlong;
    fn data_changed(&mut self) -> ();
    fn data_element_changed(&mut self, id: core::ffi::c_longlong) -> ();
    fn clear_lookup(&mut self) -> ();
}
pub trait VtkVersion {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_vtk_version(&mut self) -> &str;
    fn get_vtk_version_full(&mut self) -> &str;
    fn get_vtk_major_version(&mut self) -> core::ffi::c_int;
    fn get_vtk_minor_version(&mut self) -> core::ffi::c_int;
    fn get_vtk_build_version(&mut self) -> core::ffi::c_int;
    fn get_vtk_source_version(&mut self) -> &str;
}
pub trait VtkVoidArray {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn extended_new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn get_data_type(&mut self) -> core::ffi::c_int;
    fn get_data_type_size(&mut self) -> core::ffi::c_int;
    fn set_number_of_pointers(&mut self, number: core::ffi::c_longlong) -> ();
    fn get_number_of_pointers(&mut self) -> core::ffi::c_longlong;
    fn reset(&mut self) -> ();
    fn squeeze(&mut self) -> ();
    fn deep_copy(&mut self, va: *mut core::ffi::c_void) -> ();
}
pub trait VtkWeakPointer {
    fn get_pointer(&mut self) -> *mut core::ffi::c_void;
    fn get(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkWeakPointerBase {
    fn get_pointer(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkWeakReference {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set(&mut self, object: *mut core::ffi::c_void) -> ();
    fn get(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_window_info(&mut self, p0: &str) -> ();
    fn set_parent_info(&mut self, p0: &str) -> ();
    fn set_position(&mut self, x: core::ffi::c_int, y: core::ffi::c_int) -> ();
    fn set_size(&mut self, width: core::ffi::c_int, height: core::ffi::c_int) -> ();
    fn get_mapped(&mut self) -> core::ffi::c_int;
    fn get_show_window(&mut self) -> bool;
    fn set_show_window(&mut self, _arg: bool) -> ();
    fn show_window_on(&mut self) -> ();
    fn show_window_off(&mut self) -> ();
    fn set_use_off_screen_buffers(&mut self, _arg: bool) -> ();
    fn get_use_off_screen_buffers(&mut self) -> bool;
    fn use_off_screen_buffers_on(&mut self) -> ();
    fn use_off_screen_buffers_off(&mut self) -> ();
    fn set_erase(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_erase(&mut self) -> core::ffi::c_int;
    fn erase_on(&mut self) -> ();
    fn erase_off(&mut self) -> ();
    fn set_double_buffer(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_double_buffer(&mut self) -> core::ffi::c_int;
    fn double_buffer_on(&mut self) -> ();
    fn double_buffer_off(&mut self) -> ();
    fn set_window_name(&mut self, _arg: &str) -> ();
    fn set_icon(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn render(&mut self) -> ();
    fn release_graphics_resources(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_dpi(&mut self) -> core::ffi::c_int;
    fn set_dpi(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_dpi_min_value(&mut self) -> core::ffi::c_int;
    fn get_dpi_max_value(&mut self) -> core::ffi::c_int;
    fn detect_dpi(&mut self) -> bool;
    fn set_off_screen_rendering(&mut self, val: core::ffi::c_int) -> ();
    fn off_screen_rendering_on(&mut self) -> ();
    fn off_screen_rendering_off(&mut self) -> ();
    fn get_off_screen_rendering(&mut self) -> core::ffi::c_int;
    fn make_current(&mut self) -> ();
    fn release_current(&mut self) -> ();
    fn set_tile_scale(&mut self, _arg1: core::ffi::c_int, _arg2: core::ffi::c_int) -> ();
    fn set_tile_viewport(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
    ) -> ();
}
pub trait VtkXMLFileOutputWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn display_text(&mut self, p0: &str) -> ();
    fn display_tag(&mut self, p0: &str) -> ();
}
impl VtkAnimationCue for vtkAnimationCue {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_animation_cue_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_animation_cue_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_animation_cue_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_animation_cue_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_animation_cue_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_animation_cue_new(self.0) }
    }
    fn set_time_mode(&mut self, mode: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_animation_cue_set_time_mode(
                sself: *mut core::ffi::c_void,
                mode: core::ffi::c_int,
            );
        }
        unsafe { vtk_animation_cue_set_time_mode(self.0, mode) }
    }
    fn get_time_mode(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_animation_cue_get_time_mode(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_animation_cue_get_time_mode(self.0) }
    }
    fn set_time_mode_to_relative(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_animation_cue_set_time_mode_to_relative(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_animation_cue_set_time_mode_to_relative(self.0) }
    }
    fn set_time_mode_to_normalized(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_animation_cue_set_time_mode_to_normalized(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_animation_cue_set_time_mode_to_normalized(self.0) }
    }
    fn set_start_time(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_animation_cue_set_start_time(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_animation_cue_set_start_time(self.0, _arg) }
    }
    fn get_start_time(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_animation_cue_get_start_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_animation_cue_get_start_time(self.0) }
    }
    fn set_end_time(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_animation_cue_set_end_time(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_animation_cue_set_end_time(self.0, _arg) }
    }
    fn get_end_time(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_animation_cue_get_end_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_animation_cue_get_end_time(self.0) }
    }
    fn tick(
        &mut self,
        currenttime: core::ffi::c_double,
        deltatime: core::ffi::c_double,
        clocktime: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_animation_cue_tick(
                sself: *mut core::ffi::c_void,
                currenttime: core::ffi::c_double,
                deltatime: core::ffi::c_double,
                clocktime: core::ffi::c_double,
            );
        }
        unsafe { vtk_animation_cue_tick(self.0, currenttime, deltatime, clocktime) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_animation_cue_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_animation_cue_initialize(self.0) }
    }
    fn finalize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_animation_cue_finalize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_animation_cue_finalize(self.0) }
    }
    fn get_animation_time(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_animation_cue_get_animation_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_animation_cue_get_animation_time(self.0) }
    }
    fn get_delta_time(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_animation_cue_get_delta_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_animation_cue_get_delta_time(self.0) }
    }
    fn get_clock_time(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_animation_cue_get_clock_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_animation_cue_get_clock_time(self.0) }
    }
}
impl VtkArchiver for vtkArchiver {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_archiver_new(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_archiver_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_archiver_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_archiver_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_archiver_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_archiver_new_instance(self.0) }
    }
    fn set_archive_name(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_archiver_set_archive_name(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_archiver_set_archive_name(self.0, c__arg.as_ptr()) }
    }
    fn open_archive(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_archiver_open_archive(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_archiver_open_archive(self.0) }
    }
    fn close_archive(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_archiver_close_archive(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_archiver_close_archive(self.0) }
    }
    fn insert_into_archive(
        &mut self,
        relativePath: &str,
        data: &str,
        size: usize,
    ) -> () {
        let c_relativePath = std::ffi::CString::new(relativePath)
            .expect("CString::new failed");
        let c_data = std::ffi::CString::new(data).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_archiver_insert_into_archive(
                sself: *mut core::ffi::c_void,
                relativePath: *const core::ffi::c_char,
                data: *const core::ffi::c_char,
                size: usize,
            );
        }
        unsafe {
            vtk_archiver_insert_into_archive(
                self.0,
                c_relativePath.as_ptr(),
                c_data.as_ptr(),
                size,
            )
        }
    }
    fn contains(&mut self, relativePath: &str) -> bool {
        let c_relativePath = std::ffi::CString::new(relativePath)
            .expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_archiver_contains(
                sself: *mut core::ffi::c_void,
                relativePath: *const core::ffi::c_char,
            ) -> bool;
        }
        unsafe { vtk_archiver_contains(self.0, c_relativePath.as_ptr()) }
    }
}
impl VtkBitArray for vtkBitArray {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_bit_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_bit_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_bit_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_bit_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_bit_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_bit_array_new_instance(self.0) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                ext: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_allocate(self.0, sz, ext) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_bit_array_initialize(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_get_data_type(self.0) }
    }
    fn get_data_type_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_get_data_type_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_get_data_type_size(self.0) }
    }
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_set_number_of_tuples(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_bit_array_set_number_of_tuples(self.0, number) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_bit_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_bit_array_set_number_of_values(self.0, number) }
    }
    fn set_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_set_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_bit_array_set_tuple(self.0, i, j, source) }
    }
    fn insert_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_insert_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_bit_array_insert_tuple(self.0, i, j, source) }
    }
    fn insert_next_tuple(
        &mut self,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_bit_array_insert_next_tuple(
                sself: *mut core::ffi::c_void,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_bit_array_insert_next_tuple(self.0, j, source) }
    }
    fn remove_tuple(&mut self, id: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_remove_tuple(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_bit_array_remove_tuple(self.0, id) }
    }
    fn set_component(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_int,
        c: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_set_component(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_int,
                c: core::ffi::c_double,
            );
        }
        unsafe { vtk_bit_array_set_component(self.0, i, j, c) }
    }
    fn squeeze(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_squeeze(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_bit_array_squeeze(self.0) }
    }
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_resize(
                sself: *mut core::ffi::c_void,
                numTuples: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_resize(self.0, numTuples) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_int,
            );
        }
        unsafe { vtk_bit_array_set_value(self.0, id, value) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, i: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                i: core::ffi::c_int,
            );
        }
        unsafe { vtk_bit_array_insert_value(self.0, id, i) }
    }
    fn insert_next_value(&mut self, i: core::ffi::c_int) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_bit_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_bit_array_insert_next_value(self.0, i) }
    }
    fn insert_component(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_int,
        c: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_insert_component(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_int,
                c: core::ffi::c_double,
            );
        }
        unsafe { vtk_bit_array_insert_component(self.0, i, j, c) }
    }
    fn deep_copy(&mut self, da: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_deep_copy(
                sself: *mut core::ffi::c_void,
                da: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_bit_array_deep_copy(self.0, da) }
    }
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_set_array_free_function(
                sself: *mut core::ffi::c_void,
                callback: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_bit_array_set_array_free_function(self.0, callback) }
    }
    fn new_iterator(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_bit_array_new_iterator(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_bit_array_new_iterator(self.0) }
    }
    fn data_changed(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_data_changed(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_bit_array_data_changed(self.0) }
    }
    fn clear_lookup(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_clear_lookup(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_bit_array_clear_lookup(self.0) }
    }
}
impl VtkBitArrayIterator for vtkBitArrayIterator {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_bit_array_iterator_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_bit_array_iterator_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_bit_array_iterator_new_instance(self.0) }
    }
    fn initialize(&mut self, array: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_initialize(
                sself: *mut core::ffi::c_void,
                array: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_bit_array_iterator_initialize(self.0, array) }
    }
    fn get_array(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_get_array(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_bit_array_iterator_get_array(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_iterator_get_value(self.0, id) }
    }
    fn get_number_of_tuples(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_get_number_of_tuples(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_bit_array_iterator_get_number_of_tuples(self.0) }
    }
    fn get_number_of_values(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_get_number_of_values(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_bit_array_iterator_get_number_of_values(self.0) }
    }
    fn get_number_of_components(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_get_number_of_components(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_iterator_get_number_of_components(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_iterator_get_data_type(self.0) }
    }
    fn get_data_type_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_get_data_type_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_bit_array_iterator_get_data_type_size(self.0) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_bit_array_iterator_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_int,
            );
        }
        unsafe { vtk_bit_array_iterator_set_value(self.0, id, value) }
    }
}
impl VtkBoxMuellerRandomSequence for vtkBoxMuellerRandomSequence {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_box_mueller_random_sequence_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_box_mueller_random_sequence_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_box_mueller_random_sequence_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_box_mueller_random_sequence_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_box_mueller_random_sequence_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_box_mueller_random_sequence_new_instance(self.0) }
    }
    fn initialize(&mut self, seed: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_box_mueller_random_sequence_initialize(
                sself: *mut core::ffi::c_void,
                seed: core::ffi::c_uint,
            );
        }
        unsafe { vtk_box_mueller_random_sequence_initialize(self.0, seed) }
    }
    fn get_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_box_mueller_random_sequence_get_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_box_mueller_random_sequence_get_value(self.0) }
    }
    fn next(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_box_mueller_random_sequence_next(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_box_mueller_random_sequence_next(self.0) }
    }
    fn get_uniform_sequence(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_box_mueller_random_sequence_get_uniform_sequence(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_box_mueller_random_sequence_get_uniform_sequence(self.0) }
    }
    fn set_uniform_sequence(&mut self, uniformSequence: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_box_mueller_random_sequence_set_uniform_sequence(
                sself: *mut core::ffi::c_void,
                uniformSequence: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtk_box_mueller_random_sequence_set_uniform_sequence(self.0, uniformSequence)
        }
    }
}
impl VtkByteSwap for vtkByteSwap {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_byte_swap_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_byte_swap_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_byte_swap_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_byte_swap_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_byte_swap_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_byte_swap_new_instance(self.0) }
    }
}
impl VtkCallbackCommand for vtkCallbackCommand {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_callback_command_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_callback_command_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_callback_command_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_callback_command_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_callback_command_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_callback_command_new(self.0) }
    }
    fn set_callback(&mut self, f: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_callback_command_set_callback(
                sself: *mut core::ffi::c_void,
                f: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_callback_command_set_callback(self.0, f) }
    }
    fn set_client_data_delete_callback(&mut self, f: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_callback_command_set_client_data_delete_callback(
                sself: *mut core::ffi::c_void,
                f: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_callback_command_set_client_data_delete_callback(self.0, f) }
    }
    fn set_abort_flag_on_execute(&mut self, f: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_callback_command_set_abort_flag_on_execute(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_int,
            );
        }
        unsafe { vtk_callback_command_set_abort_flag_on_execute(self.0, f) }
    }
    fn get_abort_flag_on_execute(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_callback_command_get_abort_flag_on_execute(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_callback_command_get_abort_flag_on_execute(self.0) }
    }
    fn abort_flag_on_execute_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_callback_command_abort_flag_on_execute_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_callback_command_abort_flag_on_execute_on(self.0) }
    }
    fn abort_flag_on_execute_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_callback_command_abort_flag_on_execute_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_callback_command_abort_flag_on_execute_off(self.0) }
    }
}
impl VtkCharArray for vtkCharArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_char_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_char_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_char_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_char_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_char_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_char_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_char_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_char_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_char_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_char_array_get_data_type(self.0) }
    }
    fn set_typed_tuple(&mut self, i: core::ffi::c_longlong, tuple: &str) -> () {
        let c_tuple = std::ffi::CString::new(tuple).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_char_array_set_typed_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                tuple: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_char_array_set_typed_tuple(self.0, i, c_tuple.as_ptr()) }
    }
    fn insert_typed_tuple(&mut self, i: core::ffi::c_longlong, tuple: &str) -> () {
        let c_tuple = std::ffi::CString::new(tuple).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_char_array_insert_typed_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                tuple: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_char_array_insert_typed_tuple(self.0, i, c_tuple.as_ptr()) }
    }
    fn insert_next_typed_tuple(&mut self, tuple: &str) -> core::ffi::c_longlong {
        let c_tuple = std::ffi::CString::new(tuple).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_char_array_insert_next_typed_tuple(
                sself: *mut core::ffi::c_void,
                tuple: *const core::ffi::c_char,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_char_array_insert_next_typed_tuple(self.0, c_tuple.as_ptr()) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> &str {
        unsafe extern "C" {
            fn vtk_char_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_char_array_get_value(self.0, id) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: &str) -> () {
        let c_value = std::ffi::CString::new(value).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_char_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_char_array_set_value(self.0, id, c_value.as_ptr()) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_char_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_char_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: &str) -> () {
        let c_f = std::ffi::CString::new(f).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_char_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_char_array_insert_value(self.0, id, c_f.as_ptr()) }
    }
    fn insert_next_value(&mut self, f: &str) -> core::ffi::c_longlong {
        let c_f = std::ffi::CString::new(f).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_char_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: *const core::ffi::c_char,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_char_array_insert_next_value(self.0, c_f.as_ptr()) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_char_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_char_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_char_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_char_array_get_data_type_value_min(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn get_data_type_value_max(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_char_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_char_array_get_data_type_value_max(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}
impl VtkCollection for vtkCollection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_new(self.0) }
    }
    fn add_item(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_collection_add_item(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_collection_add_item(self.0, p0) }
    }
    fn insert_item(&mut self, i: core::ffi::c_int, p1: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_collection_insert_item(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
                p1: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_collection_insert_item(self.0, i, p1) }
    }
    fn replace_item(&mut self, i: core::ffi::c_int, p1: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_collection_replace_item(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
                p1: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_collection_replace_item(self.0, i, p1) }
    }
    fn remove_item(&mut self, i: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_collection_remove_item(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            );
        }
        unsafe { vtk_collection_remove_item(self.0, i) }
    }
    fn remove_all_items(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_collection_remove_all_items(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_collection_remove_all_items(self.0) }
    }
    fn is_item_present(&mut self, a: *mut core::ffi::c_void) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_collection_is_item_present(
                sself: *mut core::ffi::c_void,
                a: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_collection_is_item_present(self.0, a) }
    }
    fn get_number_of_items(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_collection_get_number_of_items(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_collection_get_number_of_items(self.0) }
    }
    fn init_traversal(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_collection_init_traversal(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_collection_init_traversal(self.0) }
    }
    fn get_next_item_as_object(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_get_next_item_as_object(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_get_next_item_as_object(self.0) }
    }
    fn get_item_as_object(&mut self, i: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_get_item_as_object(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_get_item_as_object(self.0, i) }
    }
    fn new_iterator(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_new_iterator(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_new_iterator(self.0) }
    }
    fn register(&mut self, o: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_collection_register(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_collection_register(self.0, o) }
    }
}
impl VtkCollectionIterator for vtkCollectionIterator {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_iterator_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_iterator_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_iterator_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_iterator_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_iterator_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_iterator_new(self.0) }
    }
    fn set_collection(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_collection_iterator_set_collection(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_collection_iterator_set_collection(self.0, p0) }
    }
    fn get_collection(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_iterator_get_collection(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_iterator_get_collection(self.0) }
    }
    fn init_traversal(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_collection_iterator_init_traversal(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_collection_iterator_init_traversal(self.0) }
    }
    fn go_to_first_item(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_collection_iterator_go_to_first_item(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_collection_iterator_go_to_first_item(self.0) }
    }
    fn go_to_next_item(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_collection_iterator_go_to_next_item(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_collection_iterator_go_to_next_item(self.0) }
    }
    fn is_done_with_traversal(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_collection_iterator_is_done_with_traversal(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_collection_iterator_is_done_with_traversal(self.0) }
    }
    fn get_current_object(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_collection_iterator_get_current_object(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_collection_iterator_get_current_object(self.0) }
    }
}
impl VtkCriticalSection for vtkCriticalSection {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_critical_section_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_critical_section_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_critical_section_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_critical_section_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_critical_section_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_critical_section_new_instance(self.0) }
    }
    fn lock(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_critical_section_lock(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_critical_section_lock(self.0) }
    }
    fn unlock(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_critical_section_unlock(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_critical_section_unlock(self.0) }
    }
}
impl VtkDataArrayCollection for vtkDataArrayCollection {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_new_instance(self.0) }
    }
    fn add_item(&mut self, ds: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_array_collection_add_item(
                sself: *mut core::ffi::c_void,
                ds: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_array_collection_add_item(self.0, ds) }
    }
    fn get_next_item(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_get_next_item(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_get_next_item(self.0) }
    }
    fn get_item(&mut self, i: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_get_item(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_get_item(self.0, i) }
    }
    fn get_number_of_items(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_data_array_collection_get_number_of_items(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_collection_get_number_of_items(self.0) }
    }
}
impl VtkDataArrayCollectionIterator for vtkDataArrayCollectionIterator {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_iterator_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_iterator_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_iterator_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_iterator_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_iterator_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_iterator_new(self.0) }
    }
    fn set_collection(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_array_collection_iterator_set_collection(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_array_collection_iterator_set_collection(self.0, p0) }
    }
    fn get_data_array(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_collection_iterator_get_data_array(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_collection_iterator_get_data_array(self.0) }
    }
}
impl VtkDataArraySelection for vtkDataArraySelection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_selection_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_selection_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_selection_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_selection_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_array_selection_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_array_selection_new(self.0) }
    }
    fn enable_array(&mut self, name: &str) -> () {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_enable_array(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_data_array_selection_enable_array(self.0, c_name.as_ptr()) }
    }
    fn disable_array(&mut self, name: &str) -> () {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_disable_array(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_data_array_selection_disable_array(self.0, c_name.as_ptr()) }
    }
    fn array_is_enabled(&mut self, name: &str) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_array_is_enabled(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_selection_array_is_enabled(self.0, c_name.as_ptr()) }
    }
    fn array_exists(&mut self, name: &str) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_array_exists(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_selection_array_exists(self.0, c_name.as_ptr()) }
    }
    fn enable_all_arrays(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_data_array_selection_enable_all_arrays(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_data_array_selection_enable_all_arrays(self.0) }
    }
    fn disable_all_arrays(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_data_array_selection_disable_all_arrays(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_array_selection_disable_all_arrays(self.0) }
    }
    fn get_number_of_arrays(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_data_array_selection_get_number_of_arrays(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_selection_get_number_of_arrays(self.0) }
    }
    fn get_number_of_arrays_enabled(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_data_array_selection_get_number_of_arrays_enabled(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_selection_get_number_of_arrays_enabled(self.0) }
    }
    fn get_array_name(&mut self, index: core::ffi::c_int) -> &str {
        unsafe extern "C" {
            fn vtk_data_array_selection_get_array_name(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_data_array_selection_get_array_name(self.0, index) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn get_array_index(&mut self, name: &str) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_get_array_index(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_selection_get_array_index(self.0, c_name.as_ptr()) }
    }
    fn get_enabled_array_index(&mut self, name: &str) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_get_enabled_array_index(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_data_array_selection_get_enabled_array_index(self.0, c_name.as_ptr())
        }
    }
    fn get_array_setting(&mut self, index: core::ffi::c_int) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_data_array_selection_get_array_setting(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_selection_get_array_setting(self.0, index) }
    }
    fn set_array_setting(&mut self, name: &str, setting: core::ffi::c_int) -> () {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_set_array_setting(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
                setting: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_data_array_selection_set_array_setting(self.0, c_name.as_ptr(), setting)
        }
    }
    fn remove_all_arrays(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_data_array_selection_remove_all_arrays(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_data_array_selection_remove_all_arrays(self.0) }
    }
    fn add_array(&mut self, name: &str, state: bool) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_add_array(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
                state: bool,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_selection_add_array(self.0, c_name.as_ptr(), state) }
    }
    fn remove_array_by_index(&mut self, index: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_data_array_selection_remove_array_by_index(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
            );
        }
        unsafe { vtk_data_array_selection_remove_array_by_index(self.0, index) }
    }
    fn remove_array_by_name(&mut self, name: &str) -> () {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_data_array_selection_remove_array_by_name(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_data_array_selection_remove_array_by_name(self.0, c_name.as_ptr()) }
    }
    fn copy_selections(&mut self, selections: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_array_selection_copy_selections(
                sself: *mut core::ffi::c_void,
                selections: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_array_selection_copy_selections(self.0, selections) }
    }
    fn union(&mut self, other: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_array_selection_union(
                sself: *mut core::ffi::c_void,
                other: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_array_selection_union(self.0, other) }
    }
    fn set_unknown_array_setting(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_data_array_selection_set_unknown_array_setting(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_data_array_selection_set_unknown_array_setting(self.0, _arg) }
    }
    fn get_unknown_array_setting(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_data_array_selection_get_unknown_array_setting(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_data_array_selection_get_unknown_array_setting(self.0) }
    }
}
impl VtkDebugLeaks for vtkDebugLeaks {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_debug_leaks_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_debug_leaks_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_debug_leaks_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_debug_leaks_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_debug_leaks_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_debug_leaks_new_instance(self.0) }
    }
    fn construct_class(&mut self, object: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_debug_leaks_construct_class(
                sself: *mut core::ffi::c_void,
                object: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_debug_leaks_construct_class(self.0, object) }
    }
    fn destruct_class(&mut self, object: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_debug_leaks_destruct_class(
                sself: *mut core::ffi::c_void,
                object: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_debug_leaks_destruct_class(self.0, object) }
    }
    fn print_current_leaks(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_debug_leaks_print_current_leaks(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_debug_leaks_print_current_leaks(self.0) }
    }
    fn get_exit_error(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_debug_leaks_get_exit_error(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_debug_leaks_get_exit_error(self.0) }
    }
    fn set_exit_error(&mut self, p0: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_debug_leaks_set_exit_error(
                sself: *mut core::ffi::c_void,
                p0: core::ffi::c_int,
            );
        }
        unsafe { vtk_debug_leaks_set_exit_error(self.0, p0) }
    }
    fn set_debug_leaks_observer(&mut self, observer: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_debug_leaks_set_debug_leaks_observer(
                sself: *mut core::ffi::c_void,
                observer: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_debug_leaks_set_debug_leaks_observer(self.0, observer) }
    }
    fn get_debug_leaks_observer(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_debug_leaks_get_debug_leaks_observer(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_debug_leaks_get_debug_leaks_observer(self.0) }
    }
}
impl VtkDoubleArray for vtkDoubleArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_double_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_double_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_double_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_double_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_double_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_double_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_double_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_double_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_double_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_double_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_double_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_double_array_get_value(self.0, id) }
    }
    fn set_value(
        &mut self,
        id: core::ffi::c_longlong,
        value: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_double_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_double,
            );
        }
        unsafe { vtk_double_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_double_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_double_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_double_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_double,
            );
        }
        unsafe { vtk_double_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_double) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_double_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_double,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_double_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_double_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_double_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_double_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_double_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_double_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_double_array_get_data_type_value_max(self.0) }
    }
}
impl VtkDynamicLoader for vtkDynamicLoader {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_dynamic_loader_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_dynamic_loader_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_dynamic_loader_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_dynamic_loader_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_dynamic_loader_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_dynamic_loader_new_instance(self.0) }
    }
    fn lib_prefix(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_dynamic_loader_lib_prefix(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_dynamic_loader_lib_prefix(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn lib_extension(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_dynamic_loader_lib_extension(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_dynamic_loader_lib_extension(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn last_error(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_dynamic_loader_last_error(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_dynamic_loader_last_error(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}
impl VtkEventDataDevice3D for vtkEventDataDevice3D {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_data_device_3_d_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_data_device_3_d_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_data_device_3_d_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_data_device_3_d_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_data_device_3_d_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_data_device_3_d_new(self.0) }
    }
    fn set_track_pad_position(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_event_data_device_3_d_set_track_pad_position(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
            );
        }
        unsafe { vtk_event_data_device_3_d_set_track_pad_position(self.0, x, y) }
    }
}
impl VtkEventDataForDevice for vtkEventDataForDevice {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_data_for_device_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_data_for_device_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_data_for_device_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_data_for_device_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_data_for_device_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_data_for_device_new(self.0) }
    }
}
impl VtkEventForwarderCommand for vtkEventForwarderCommand {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_forwarder_command_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_forwarder_command_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_forwarder_command_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_forwarder_command_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_event_forwarder_command_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_event_forwarder_command_new(self.0) }
    }
    fn set_target(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_event_forwarder_command_set_target(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_event_forwarder_command_set_target(self.0, obj) }
    }
}
impl VtkFileOutputWindow for vtkFileOutputWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_file_output_window_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_file_output_window_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_file_output_window_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_file_output_window_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_file_output_window_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_file_output_window_new(self.0) }
    }
    fn display_text(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_file_output_window_display_text(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_file_output_window_display_text(self.0, c_p0.as_ptr()) }
    }
    fn set_file_name(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_file_output_window_set_file_name(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_file_output_window_set_file_name(self.0, c__arg.as_ptr()) }
    }
    fn set_flush(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_file_output_window_set_flush(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_file_output_window_set_flush(self.0, _arg) }
    }
    fn get_flush(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_file_output_window_get_flush(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_file_output_window_get_flush(self.0) }
    }
    fn flush_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_file_output_window_flush_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_file_output_window_flush_on(self.0) }
    }
    fn flush_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_file_output_window_flush_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_file_output_window_flush_off(self.0) }
    }
    fn set_append(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_file_output_window_set_append(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_file_output_window_set_append(self.0, _arg) }
    }
    fn get_append(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_file_output_window_get_append(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_file_output_window_get_append(self.0) }
    }
    fn append_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_file_output_window_append_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_file_output_window_append_on(self.0) }
    }
    fn append_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_file_output_window_append_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_file_output_window_append_off(self.0) }
    }
}
impl VtkFloatArray for vtkFloatArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_float_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_float_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_float_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_float_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_float_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_float_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_float_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_float_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_float_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_float_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_float {
        unsafe extern "C" {
            fn vtk_float_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_float;
        }
        unsafe { vtk_float_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_float) -> () {
        unsafe extern "C" {
            fn vtk_float_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_float,
            );
        }
        unsafe { vtk_float_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_float_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_float_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_float) -> () {
        unsafe extern "C" {
            fn vtk_float_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_float,
            );
        }
        unsafe { vtk_float_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_float) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_float_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_float,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_float_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_float_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_float_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_float {
        unsafe extern "C" {
            fn vtk_float_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_float;
        }
        unsafe { vtk_float_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_float {
        unsafe extern "C" {
            fn vtk_float_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_float;
        }
        unsafe { vtk_float_array_get_data_type_value_max(self.0) }
    }
}
impl VtkGarbageCollector for vtkGarbageCollector {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_garbage_collector_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_garbage_collector_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_garbage_collector_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_garbage_collector_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_garbage_collector_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_garbage_collector_new(self.0) }
    }
    fn collect(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_garbage_collector_collect(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_garbage_collector_collect(self.0) }
    }
    fn deferred_collection_push(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_garbage_collector_deferred_collection_push(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_garbage_collector_deferred_collection_push(self.0) }
    }
    fn deferred_collection_pop(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_garbage_collector_deferred_collection_pop(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_garbage_collector_deferred_collection_pop(self.0) }
    }
    fn set_global_debug_flag(&mut self, flag: bool) -> () {
        unsafe extern "C" {
            fn vtk_garbage_collector_set_global_debug_flag(
                sself: *mut core::ffi::c_void,
                flag: bool,
            );
        }
        unsafe { vtk_garbage_collector_set_global_debug_flag(self.0, flag) }
    }
    fn get_global_debug_flag(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_garbage_collector_get_global_debug_flag(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_garbage_collector_get_global_debug_flag(self.0) }
    }
}
impl VtkIdList for vtkIdList {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_list_new(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_list_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_list_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_list_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_list_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_list_new_instance(self.0) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_id_list_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_id_list_initialize(self.0) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        strategy: core::ffi::c_int,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_id_list_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                strategy: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_id_list_allocate(self.0, sz, strategy) }
    }
    fn get_number_of_ids(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_list_get_number_of_ids(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_list_get_number_of_ids(self.0) }
    }
    fn get_id(&mut self, i: core::ffi::c_longlong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_list_get_id(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_list_get_id(self.0, i) }
    }
    fn find_id_location(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_list_find_id_location(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_list_find_id_location(self.0, id) }
    }
    fn set_number_of_ids(&mut self, number: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_id_list_set_number_of_ids(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_id_list_set_number_of_ids(self.0, number) }
    }
    fn set_id(&mut self, i: core::ffi::c_longlong, vtkid: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_id_list_set_id(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                vtkid: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_id_list_set_id(self.0, i, vtkid) }
    }
    fn insert_id(
        &mut self,
        i: core::ffi::c_longlong,
        vtkid: core::ffi::c_longlong,
    ) -> () {
        unsafe extern "C" {
            fn vtk_id_list_insert_id(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                vtkid: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_id_list_insert_id(self.0, i, vtkid) }
    }
    fn insert_next_id(&mut self, vtkid: core::ffi::c_longlong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_list_insert_next_id(
                sself: *mut core::ffi::c_void,
                vtkid: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_list_insert_next_id(self.0, vtkid) }
    }
    fn insert_unique_id(
        &mut self,
        vtkid: core::ffi::c_longlong,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_list_insert_unique_id(
                sself: *mut core::ffi::c_void,
                vtkid: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_list_insert_unique_id(self.0, vtkid) }
    }
    fn sort(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_id_list_sort(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_id_list_sort(self.0) }
    }
    fn fill(&mut self, value: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_id_list_fill(
                sself: *mut core::ffi::c_void,
                value: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_id_list_fill(self.0, value) }
    }
    fn reset(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_id_list_reset(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_id_list_reset(self.0) }
    }
    fn squeeze(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_id_list_squeeze(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_id_list_squeeze(self.0) }
    }
    fn deep_copy(&mut self, ids: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_id_list_deep_copy(
                sself: *mut core::ffi::c_void,
                ids: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_id_list_deep_copy(self.0, ids) }
    }
    fn delete_id(&mut self, vtkid: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_id_list_delete_id(
                sself: *mut core::ffi::c_void,
                vtkid: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_id_list_delete_id(self.0, vtkid) }
    }
    fn is_id(&mut self, vtkid: core::ffi::c_longlong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_list_is_id(
                sself: *mut core::ffi::c_void,
                vtkid: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_list_is_id(self.0, vtkid) }
    }
    fn intersect_with(&mut self, otherIds: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_id_list_intersect_with(
                sself: *mut core::ffi::c_void,
                otherIds: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_id_list_intersect_with(self.0, otherIds) }
    }
}
impl VtkIdListCollection for vtkIdListCollection {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_list_collection_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_list_collection_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_list_collection_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_list_collection_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_list_collection_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_list_collection_new_instance(self.0) }
    }
    fn add_item(&mut self, ds: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_id_list_collection_add_item(
                sself: *mut core::ffi::c_void,
                ds: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_id_list_collection_add_item(self.0, ds) }
    }
    fn get_next_item(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_list_collection_get_next_item(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_list_collection_get_next_item(self.0) }
    }
    fn get_item(&mut self, i: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_list_collection_get_item(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_list_collection_get_item(self.0, i) }
    }
    fn get_number_of_items(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_id_list_collection_get_number_of_items(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_id_list_collection_get_number_of_items(self.0) }
    }
}
impl VtkIdTypeArray for vtkIdTypeArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_type_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_type_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_type_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_type_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_type_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_type_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_type_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_type_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_id_type_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_id_type_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_type_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_type_array_get_value(self.0, id) }
    }
    fn set_value(
        &mut self,
        id: core::ffi::c_longlong,
        value: core::ffi::c_longlong,
    ) -> () {
        unsafe extern "C" {
            fn vtk_id_type_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_id_type_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_id_type_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_id_type_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(
        &mut self,
        id: core::ffi::c_longlong,
        f: core::ffi::c_longlong,
    ) -> () {
        unsafe extern "C" {
            fn vtk_id_type_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_id_type_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_longlong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_type_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_type_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_id_type_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_id_type_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_type_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_type_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_id_type_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_id_type_array_get_data_type_value_max(self.0) }
    }
}
impl VtkInformation for vtkInformation {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_new_instance(self.0) }
    }
    fn modified(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_information_modified(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_information_modified(self.0) }
    }
    fn clear(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_information_clear(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_information_clear(self.0) }
    }
    fn get_number_of_keys(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_information_get_number_of_keys(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_information_get_number_of_keys(self.0) }
    }
    fn copy(&mut self, from: *mut core::ffi::c_void, deep: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_information_copy(
                sself: *mut core::ffi::c_void,
                from: *mut core::ffi::c_void,
                deep: core::ffi::c_int,
            );
        }
        unsafe { vtk_information_copy(self.0, from, deep) }
    }
    fn append(&mut self, from: *mut core::ffi::c_void, deep: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_information_append(
                sself: *mut core::ffi::c_void,
                from: *mut core::ffi::c_void,
                deep: core::ffi::c_int,
            );
        }
        unsafe { vtk_information_append(self.0, from, deep) }
    }
    fn copy_entry(
        &mut self,
        from: *mut core::ffi::c_void,
        key: *mut core::ffi::c_void,
        deep: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_information_copy_entry(
                sself: *mut core::ffi::c_void,
                from: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
                deep: core::ffi::c_int,
            );
        }
        unsafe { vtk_information_copy_entry(self.0, from, key, deep) }
    }
    fn copy_entries(
        &mut self,
        from: *mut core::ffi::c_void,
        key: *mut core::ffi::c_void,
        deep: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_information_copy_entries(
                sself: *mut core::ffi::c_void,
                from: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
                deep: core::ffi::c_int,
            );
        }
        unsafe { vtk_information_copy_entries(self.0, from, key, deep) }
    }
    fn has(&mut self, key: *mut core::ffi::c_void) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_information_has(
                sself: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_information_has(self.0, key) }
    }
    fn remove(&mut self, key: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_remove(
                sself: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_remove(self.0, key) }
    }
    fn set(&mut self, key: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_set(
                sself: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_set(self.0, key) }
    }
    fn get(&mut self, key: *mut core::ffi::c_void) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_information_get(
                sself: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_information_get(self.0, key) }
    }
    fn length(&mut self, key: *mut core::ffi::c_void) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_information_length(
                sself: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_information_length(self.0, key) }
    }
    fn append_unique(
        &mut self,
        key: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_information_append_unique(
                sself: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
                value: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_append_unique(self.0, key, value) }
    }
    fn get_key(&mut self, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_get_key(
                sself: *mut core::ffi::c_void,
                key: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_get_key(self.0, key) }
    }
    fn register(&mut self, o: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_register(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_register(self.0, o) }
    }
    fn set_request(&mut self, request: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_set_request(
                sself: *mut core::ffi::c_void,
                request: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_set_request(self.0, request) }
    }
    fn get_request(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_get_request(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_get_request(self.0) }
    }
}
impl VtkInformationIterator for vtkInformationIterator {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_iterator_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_iterator_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_iterator_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_iterator_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_iterator_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_iterator_new_instance(self.0) }
    }
    fn set_information(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_iterator_set_information(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_iterator_set_information(self.0, p0) }
    }
    fn get_information(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_iterator_get_information(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_iterator_get_information(self.0) }
    }
    fn set_information_weak(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_iterator_set_information_weak(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_iterator_set_information_weak(self.0, p0) }
    }
    fn init_traversal(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_information_iterator_init_traversal(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_information_iterator_init_traversal(self.0) }
    }
    fn go_to_first_item(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_information_iterator_go_to_first_item(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_information_iterator_go_to_first_item(self.0) }
    }
    fn go_to_next_item(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_information_iterator_go_to_next_item(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_information_iterator_go_to_next_item(self.0) }
    }
    fn is_done_with_traversal(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_information_iterator_is_done_with_traversal(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_information_iterator_is_done_with_traversal(self.0) }
    }
    fn get_current_key(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_iterator_get_current_key(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_iterator_get_current_key(self.0) }
    }
}
impl VtkInformationKeyLookup for vtkInformationKeyLookup {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_key_lookup_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_key_lookup_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_key_lookup_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_key_lookup_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_key_lookup_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_key_lookup_new_instance(self.0) }
    }
    fn find(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        let c_location = std::ffi::CString::new(location).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_information_key_lookup_find(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
                location: *const core::ffi::c_char,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_information_key_lookup_find(self.0, c_name.as_ptr(), c_location.as_ptr())
        }
    }
}
impl VtkInformationVector for vtkInformationVector {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_vector_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_vector_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_vector_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_vector_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_vector_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_vector_new_instance(self.0) }
    }
    fn get_number_of_information_objects(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_information_vector_get_number_of_information_objects(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_information_vector_get_number_of_information_objects(self.0) }
    }
    fn set_number_of_information_objects(&mut self, n: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_information_vector_set_number_of_information_objects(
                sself: *mut core::ffi::c_void,
                n: core::ffi::c_int,
            );
        }
        unsafe { vtk_information_vector_set_number_of_information_objects(self.0, n) }
    }
    fn set_information_object(
        &mut self,
        index: core::ffi::c_int,
        info: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_information_vector_set_information_object(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
                info: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_vector_set_information_object(self.0, index, info) }
    }
    fn get_information_object(
        &mut self,
        index: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_information_vector_get_information_object(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_information_vector_get_information_object(self.0, index) }
    }
    fn append(&mut self, info: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_vector_append(
                sself: *mut core::ffi::c_void,
                info: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_vector_append(self.0, info) }
    }
    fn remove(&mut self, info: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_vector_remove(
                sself: *mut core::ffi::c_void,
                info: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_vector_remove(self.0, info) }
    }
    fn register(&mut self, o: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_information_vector_register(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_information_vector_register(self.0, o) }
    }
    fn copy(&mut self, from: *mut core::ffi::c_void, deep: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_information_vector_copy(
                sself: *mut core::ffi::c_void,
                from: *mut core::ffi::c_void,
                deep: core::ffi::c_int,
            );
        }
        unsafe { vtk_information_vector_copy(self.0, from, deep) }
    }
}
impl VtkIntArray for vtkIntArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_int_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_int_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_int_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_int_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_int_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_int_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_int_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_int_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_int_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_int_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_int_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_int_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_int_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_int,
            );
        }
        unsafe { vtk_int_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_int_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_int_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_int_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_int,
            );
        }
        unsafe { vtk_int_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_int) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_int_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_int,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_int_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_int_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_int_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_int_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_int_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_int_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_int_array_get_data_type_value_max(self.0) }
    }
}
impl VtkLongArray for vtkLongArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_long_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_long_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_long {
        unsafe extern "C" {
            fn vtk_long_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_long;
        }
        unsafe { vtk_long_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_long) -> () {
        unsafe extern "C" {
            fn vtk_long_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_long,
            );
        }
        unsafe { vtk_long_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_long_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_long_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_long) -> () {
        unsafe extern "C" {
            fn vtk_long_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_long,
            );
        }
        unsafe { vtk_long_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_long) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_long_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_long,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_long_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_long {
        unsafe extern "C" {
            fn vtk_long_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_long;
        }
        unsafe { vtk_long_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_long {
        unsafe extern "C" {
            fn vtk_long_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_long;
        }
        unsafe { vtk_long_array_get_data_type_value_max(self.0) }
    }
}
impl VtkLongLongArray for vtkLongLongArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_long_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_long_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_long_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_long_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_long_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_long_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_long_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_long_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_long_long_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_long_long_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_long_long_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_long_long_array_get_value(self.0, id) }
    }
    fn set_value(
        &mut self,
        id: core::ffi::c_longlong,
        value: core::ffi::c_longlong,
    ) -> () {
        unsafe extern "C" {
            fn vtk_long_long_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_long_long_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_long_long_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_long_long_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(
        &mut self,
        id: core::ffi::c_longlong,
        f: core::ffi::c_longlong,
    ) -> () {
        unsafe extern "C" {
            fn vtk_long_long_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_long_long_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_longlong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_long_long_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_longlong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_long_long_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_long_long_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_long_long_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_long_long_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_long_long_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_long_long_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_long_long_array_get_data_type_value_max(self.0) }
    }
}
impl VtkLookupTable for vtkLookupTable {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_lookup_table_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_lookup_table_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_lookup_table_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_lookup_table_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_lookup_table_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_lookup_table_new_instance(self.0) }
    }
    fn is_opaque(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_lookup_table_is_opaque(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_lookup_table_is_opaque(self.0) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_int,
        ext: core::ffi::c_int,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_lookup_table_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_int,
                ext: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_lookup_table_allocate(self.0, sz, ext) }
    }
    fn build(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_build(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_build(self.0) }
    }
    fn force_build(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_force_build(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_force_build(self.0) }
    }
    fn build_special_colors(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_build_special_colors(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_build_special_colors(self.0) }
    }
    fn set_ramp(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_ramp(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_lookup_table_set_ramp(self.0, _arg) }
    }
    fn set_ramp_to_linear(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_ramp_to_linear(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_set_ramp_to_linear(self.0) }
    }
    fn set_ramp_to_s_curve(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_ramp_to_s_curve(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_set_ramp_to_s_curve(self.0) }
    }
    fn set_ramp_to_sqrt(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_ramp_to_sqrt(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_set_ramp_to_sqrt(self.0) }
    }
    fn get_ramp(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_lookup_table_get_ramp(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_lookup_table_get_ramp(self.0) }
    }
    fn set_scale(&mut self, scale: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_scale(
                sself: *mut core::ffi::c_void,
                scale: core::ffi::c_int,
            );
        }
        unsafe { vtk_lookup_table_set_scale(self.0, scale) }
    }
    fn set_scale_to_linear(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_scale_to_linear(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_set_scale_to_linear(self.0) }
    }
    fn set_scale_to_log_10(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_scale_to_log_10(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_set_scale_to_log_10(self.0) }
    }
    fn get_scale(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_lookup_table_get_scale(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_lookup_table_get_scale(self.0) }
    }
    fn set_table_range(
        &mut self,
        min: core::ffi::c_double,
        max: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_table_range(
                sself: *mut core::ffi::c_void,
                min: core::ffi::c_double,
                max: core::ffi::c_double,
            );
        }
        unsafe { vtk_lookup_table_set_table_range(self.0, min, max) }
    }
    fn set_hue_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_hue_range(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
            );
        }
        unsafe { vtk_lookup_table_set_hue_range(self.0, _arg1, _arg2) }
    }
    fn set_saturation_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_saturation_range(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
            );
        }
        unsafe { vtk_lookup_table_set_saturation_range(self.0, _arg1, _arg2) }
    }
    fn set_value_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_value_range(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
            );
        }
        unsafe { vtk_lookup_table_set_value_range(self.0, _arg1, _arg2) }
    }
    fn set_alpha_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_alpha_range(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
            );
        }
        unsafe { vtk_lookup_table_set_alpha_range(self.0, _arg1, _arg2) }
    }
    fn set_nan_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_nan_color(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
                _arg4: core::ffi::c_double,
            );
        }
        unsafe { vtk_lookup_table_set_nan_color(self.0, _arg1, _arg2, _arg3, _arg4) }
    }
    fn set_below_range_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_below_range_color(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
                _arg4: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_lookup_table_set_below_range_color(self.0, _arg1, _arg2, _arg3, _arg4)
        }
    }
    fn set_use_below_range_color(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_use_below_range_color(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_lookup_table_set_use_below_range_color(self.0, _arg) }
    }
    fn get_use_below_range_color(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_lookup_table_get_use_below_range_color(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_lookup_table_get_use_below_range_color(self.0) }
    }
    fn use_below_range_color_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_use_below_range_color_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_use_below_range_color_on(self.0) }
    }
    fn use_below_range_color_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_use_below_range_color_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_use_below_range_color_off(self.0) }
    }
    fn set_above_range_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_above_range_color(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
                _arg4: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_lookup_table_set_above_range_color(self.0, _arg1, _arg2, _arg3, _arg4)
        }
    }
    fn set_use_above_range_color(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_use_above_range_color(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_lookup_table_set_use_above_range_color(self.0, _arg) }
    }
    fn get_use_above_range_color(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_lookup_table_get_use_above_range_color(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_lookup_table_get_use_above_range_color(self.0) }
    }
    fn use_above_range_color_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_use_above_range_color_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_use_above_range_color_on(self.0) }
    }
    fn use_above_range_color_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_use_above_range_color_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_lookup_table_use_above_range_color_off(self.0) }
    }
    fn get_opacity(&mut self, v: core::ffi::c_double) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_lookup_table_get_opacity(
                sself: *mut core::ffi::c_void,
                v: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_lookup_table_get_opacity(self.0, v) }
    }
    fn get_index(&mut self, v: core::ffi::c_double) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_lookup_table_get_index(
                sself: *mut core::ffi::c_void,
                v: core::ffi::c_double,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_lookup_table_get_index(self.0, v) }
    }
    fn set_number_of_table_values(&mut self, number: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_number_of_table_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_lookup_table_set_number_of_table_values(self.0, number) }
    }
    fn get_number_of_table_values(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_lookup_table_get_number_of_table_values(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_lookup_table_get_number_of_table_values(self.0) }
    }
    fn set_table_value(
        &mut self,
        indx: core::ffi::c_longlong,
        r: core::ffi::c_double,
        g: core::ffi::c_double,
        b: core::ffi::c_double,
        a: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_table_value(
                sself: *mut core::ffi::c_void,
                indx: core::ffi::c_longlong,
                r: core::ffi::c_double,
                g: core::ffi::c_double,
                b: core::ffi::c_double,
                a: core::ffi::c_double,
            );
        }
        unsafe { vtk_lookup_table_set_table_value(self.0, indx, r, g, b, a) }
    }
    fn set_number_of_colors(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_number_of_colors(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_lookup_table_set_number_of_colors(self.0, _arg) }
    }
    fn get_number_of_colors_min_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_lookup_table_get_number_of_colors_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_lookup_table_get_number_of_colors_min_value(self.0) }
    }
    fn get_number_of_colors_max_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_lookup_table_get_number_of_colors_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_lookup_table_get_number_of_colors_max_value(self.0) }
    }
    fn get_number_of_colors(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_lookup_table_get_number_of_colors(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_lookup_table_get_number_of_colors(self.0) }
    }
    fn set_table(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_set_table(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_lookup_table_set_table(self.0, p0) }
    }
    fn get_table(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_lookup_table_get_table(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_lookup_table_get_table(self.0) }
    }
    fn deep_copy(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_lookup_table_deep_copy(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_lookup_table_deep_copy(self.0, obj) }
    }
    fn using_log_scale(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_lookup_table_using_log_scale(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_lookup_table_using_log_scale(self.0) }
    }
}
impl VtkMath for vtkMath {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_math_new(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_math_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_math_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_math_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_math_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_math_new_instance(self.0) }
    }
    fn pi(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_pi(sself: *mut core::ffi::c_void) -> core::ffi::c_double;
        }
        unsafe { vtk_math_pi(self.0) }
    }
    fn radians_from_degrees(
        &mut self,
        degrees: core::ffi::c_float,
    ) -> core::ffi::c_float {
        unsafe extern "C" {
            fn vtk_math_radians_from_degrees(
                sself: *mut core::ffi::c_void,
                degrees: core::ffi::c_float,
            ) -> core::ffi::c_float;
        }
        unsafe { vtk_math_radians_from_degrees(self.0, degrees) }
    }
    fn degrees_from_radians(
        &mut self,
        radians: core::ffi::c_float,
    ) -> core::ffi::c_float {
        unsafe extern "C" {
            fn vtk_math_degrees_from_radians(
                sself: *mut core::ffi::c_void,
                radians: core::ffi::c_float,
            ) -> core::ffi::c_float;
        }
        unsafe { vtk_math_degrees_from_radians(self.0, radians) }
    }
    fn round(&mut self, f: core::ffi::c_float) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_round(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_float,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_math_round(self.0, f) }
    }
    fn floor(&mut self, x: core::ffi::c_double) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_floor(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_math_floor(self.0, x) }
    }
    fn ceil(&mut self, x: core::ffi::c_double) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_ceil(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_math_ceil(self.0, x) }
    }
    fn ceil_log_2(&mut self, x: core::ffi::c_ulonglong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_ceil_log_2(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_ulonglong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_math_ceil_log_2(self.0, x) }
    }
    fn is_power_of_two(&mut self, x: core::ffi::c_ulonglong) -> bool {
        unsafe extern "C" {
            fn vtk_math_is_power_of_two(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_ulonglong,
            ) -> bool;
        }
        unsafe { vtk_math_is_power_of_two(self.0, x) }
    }
    fn nearest_power_of_two(&mut self, x: core::ffi::c_int) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_nearest_power_of_two(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_math_nearest_power_of_two(self.0, x) }
    }
    fn factorial(&mut self, N: core::ffi::c_int) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_math_factorial(
                sself: *mut core::ffi::c_void,
                N: core::ffi::c_int,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_math_factorial(self.0, N) }
    }
    fn binomial(
        &mut self,
        m: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_math_binomial(
                sself: *mut core::ffi::c_void,
                m: core::ffi::c_int,
                n: core::ffi::c_int,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_math_binomial(self.0, m, n) }
    }
    fn random_seed(&mut self, s: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_math_random_seed(sself: *mut core::ffi::c_void, s: core::ffi::c_int);
        }
        unsafe { vtk_math_random_seed(self.0, s) }
    }
    fn get_seed(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_get_seed(sself: *mut core::ffi::c_void) -> core::ffi::c_int;
        }
        unsafe { vtk_math_get_seed(self.0) }
    }
    fn random(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_random(sself: *mut core::ffi::c_void) -> core::ffi::c_double;
        }
        unsafe { vtk_math_random(self.0) }
    }
    fn gaussian(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_gaussian(sself: *mut core::ffi::c_void) -> core::ffi::c_double;
        }
        unsafe { vtk_math_gaussian(self.0) }
    }
    fn gaussian_amplitude(
        &mut self,
        variance: core::ffi::c_double,
        distanceFromMean: core::ffi::c_double,
    ) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_gaussian_amplitude(
                sself: *mut core::ffi::c_void,
                variance: core::ffi::c_double,
                distanceFromMean: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_math_gaussian_amplitude(self.0, variance, distanceFromMean) }
    }
    fn gaussian_weight(
        &mut self,
        variance: core::ffi::c_double,
        distanceFromMean: core::ffi::c_double,
    ) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_gaussian_weight(
                sself: *mut core::ffi::c_void,
                variance: core::ffi::c_double,
                distanceFromMean: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_math_gaussian_weight(self.0, variance, distanceFromMean) }
    }
    fn determinant_2_x_2(
        &mut self,
        a: core::ffi::c_double,
        b: core::ffi::c_double,
        c: core::ffi::c_double,
        d: core::ffi::c_double,
    ) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_determinant_2_x_2(
                sself: *mut core::ffi::c_void,
                a: core::ffi::c_double,
                b: core::ffi::c_double,
                c: core::ffi::c_double,
                d: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_math_determinant_2_x_2(self.0, a, b, c, d) }
    }
    fn determinant_3_x_3(
        &mut self,
        a1: core::ffi::c_double,
        a2: core::ffi::c_double,
        a3: core::ffi::c_double,
        b1: core::ffi::c_double,
        b2: core::ffi::c_double,
        b3: core::ffi::c_double,
        c1: core::ffi::c_double,
        c2: core::ffi::c_double,
        c3: core::ffi::c_double,
    ) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_determinant_3_x_3(
                sself: *mut core::ffi::c_void,
                a1: core::ffi::c_double,
                a2: core::ffi::c_double,
                a3: core::ffi::c_double,
                b1: core::ffi::c_double,
                b2: core::ffi::c_double,
                b3: core::ffi::c_double,
                c1: core::ffi::c_double,
                c2: core::ffi::c_double,
                c3: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_math_determinant_3_x_3(self.0, a1, a2, a3, b1, b2, b3, c1, c2, c3) }
    }
    fn solve_linear_system_gepp_2_x_2(
        &mut self,
        a00: core::ffi::c_double,
        a01: core::ffi::c_double,
        a10: core::ffi::c_double,
        a11: core::ffi::c_double,
        b0: core::ffi::c_double,
        b1: core::ffi::c_double,
        x0: &mut core::ffi::c_double,
        x1: &mut core::ffi::c_double,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_solve_linear_system_gepp_2_x_2(
                sself: *mut core::ffi::c_void,
                a00: core::ffi::c_double,
                a01: core::ffi::c_double,
                a10: core::ffi::c_double,
                a11: core::ffi::c_double,
                b0: core::ffi::c_double,
                b1: core::ffi::c_double,
                x0: &mut core::ffi::c_double,
                x1: &mut core::ffi::c_double,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_math_solve_linear_system_gepp_2_x_2(
                self.0,
                a00,
                a01,
                a10,
                a11,
                b0,
                b1,
                x0,
                x1,
            )
        }
    }
    fn get_scalar_type_fitting_range(
        &mut self,
        range_min: core::ffi::c_double,
        range_max: core::ffi::c_double,
        scale: core::ffi::c_double,
        shift: core::ffi::c_double,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_get_scalar_type_fitting_range(
                sself: *mut core::ffi::c_void,
                range_min: core::ffi::c_double,
                range_max: core::ffi::c_double,
                scale: core::ffi::c_double,
                shift: core::ffi::c_double,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_math_get_scalar_type_fitting_range(
                self.0,
                range_min,
                range_max,
                scale,
                shift,
            )
        }
    }
    fn inf(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_inf(sself: *mut core::ffi::c_void) -> core::ffi::c_double;
        }
        unsafe { vtk_math_inf(self.0) }
    }
    fn neg_inf(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_neg_inf(sself: *mut core::ffi::c_void) -> core::ffi::c_double;
        }
        unsafe { vtk_math_neg_inf(self.0) }
    }
    fn nan(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_math_nan(sself: *mut core::ffi::c_void) -> core::ffi::c_double;
        }
        unsafe { vtk_math_nan(self.0) }
    }
    fn is_inf(&mut self, x: core::ffi::c_double) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_is_inf(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_math_is_inf(self.0, x) }
    }
    fn is_nan(&mut self, x: core::ffi::c_double) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_math_is_nan(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_math_is_nan(self.0, x) }
    }
    fn is_finite(&mut self, x: core::ffi::c_double) -> bool {
        unsafe extern "C" {
            fn vtk_math_is_finite(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
            ) -> bool;
        }
        unsafe { vtk_math_is_finite(self.0, x) }
    }
}
impl VtkMersenneTwister for vtkMersenneTwister {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_mersenne_twister_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_mersenne_twister_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_mersenne_twister_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_mersenne_twister_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_mersenne_twister_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_mersenne_twister_new_instance(self.0) }
    }
    fn initialize(&mut self, seed: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_mersenne_twister_initialize(
                sself: *mut core::ffi::c_void,
                seed: core::ffi::c_uint,
            );
        }
        unsafe { vtk_mersenne_twister_initialize(self.0, seed) }
    }
    fn initialize_new_sequence(
        &mut self,
        seed: core::ffi::c_uint,
        p: core::ffi::c_int,
    ) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_mersenne_twister_initialize_new_sequence(
                sself: *mut core::ffi::c_void,
                seed: core::ffi::c_uint,
                p: core::ffi::c_int,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_mersenne_twister_initialize_new_sequence(self.0, seed, p) }
    }
    fn initialize_sequence(
        &mut self,
        id: core::ffi::c_uint,
        seed: core::ffi::c_uint,
        p: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_mersenne_twister_initialize_sequence(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_uint,
                seed: core::ffi::c_uint,
                p: core::ffi::c_int,
            );
        }
        unsafe { vtk_mersenne_twister_initialize_sequence(self.0, id, seed, p) }
    }
    fn get_value(&mut self, id: core::ffi::c_uint) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_mersenne_twister_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_uint,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_mersenne_twister_get_value(self.0, id) }
    }
    fn next(&mut self, id: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_mersenne_twister_next(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_uint,
            );
        }
        unsafe { vtk_mersenne_twister_next(self.0, id) }
    }
}
impl VtkMinimalStandardRandomSequence for vtkMinimalStandardRandomSequence {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_minimal_standard_random_sequence_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_minimal_standard_random_sequence_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_minimal_standard_random_sequence_new_instance(self.0) }
    }
    fn initialize(&mut self, seed: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_initialize(
                sself: *mut core::ffi::c_void,
                seed: core::ffi::c_uint,
            );
        }
        unsafe { vtk_minimal_standard_random_sequence_initialize(self.0, seed) }
    }
    fn set_seed(&mut self, value: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_set_seed(
                sself: *mut core::ffi::c_void,
                value: core::ffi::c_int,
            );
        }
        unsafe { vtk_minimal_standard_random_sequence_set_seed(self.0, value) }
    }
    fn set_seed_only(&mut self, value: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_set_seed_only(
                sself: *mut core::ffi::c_void,
                value: core::ffi::c_int,
            );
        }
        unsafe { vtk_minimal_standard_random_sequence_set_seed_only(self.0, value) }
    }
    fn get_seed(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_get_seed(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_minimal_standard_random_sequence_get_seed(self.0) }
    }
    fn get_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_get_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_minimal_standard_random_sequence_get_value(self.0) }
    }
    fn next(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_next(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_minimal_standard_random_sequence_next(self.0) }
    }
    fn get_range_value(
        &mut self,
        rangeMin: core::ffi::c_double,
        rangeMax: core::ffi::c_double,
    ) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_get_range_value(
                sself: *mut core::ffi::c_void,
                rangeMin: core::ffi::c_double,
                rangeMax: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe {
            vtk_minimal_standard_random_sequence_get_range_value(
                self.0,
                rangeMin,
                rangeMax,
            )
        }
    }
    fn get_next_range_value(
        &mut self,
        rangeMin: core::ffi::c_double,
        rangeMax: core::ffi::c_double,
    ) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_minimal_standard_random_sequence_get_next_range_value(
                sself: *mut core::ffi::c_void,
                rangeMin: core::ffi::c_double,
                rangeMax: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe {
            vtk_minimal_standard_random_sequence_get_next_range_value(
                self.0,
                rangeMin,
                rangeMax,
            )
        }
    }
}
impl VtkMultiThreader for vtkMultiThreader {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_threader_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_threader_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_threader_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_threader_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_threader_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_threader_new_instance(self.0) }
    }
    fn set_number_of_threads(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_multi_threader_set_number_of_threads(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_multi_threader_set_number_of_threads(self.0, _arg) }
    }
    fn get_number_of_threads_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_multi_threader_get_number_of_threads_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_multi_threader_get_number_of_threads_min_value(self.0) }
    }
    fn get_number_of_threads_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_multi_threader_get_number_of_threads_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_multi_threader_get_number_of_threads_max_value(self.0) }
    }
    fn get_number_of_threads(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_multi_threader_get_number_of_threads(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_multi_threader_get_number_of_threads(self.0) }
    }
    fn get_global_static_maximum_number_of_threads(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_multi_threader_get_global_static_maximum_number_of_threads(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_multi_threader_get_global_static_maximum_number_of_threads(self.0) }
    }
    fn set_global_maximum_number_of_threads(&mut self, val: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_multi_threader_set_global_maximum_number_of_threads(
                sself: *mut core::ffi::c_void,
                val: core::ffi::c_int,
            );
        }
        unsafe { vtk_multi_threader_set_global_maximum_number_of_threads(self.0, val) }
    }
    fn get_global_maximum_number_of_threads(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_multi_threader_get_global_maximum_number_of_threads(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_multi_threader_get_global_maximum_number_of_threads(self.0) }
    }
    fn set_global_default_number_of_threads(&mut self, val: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_multi_threader_set_global_default_number_of_threads(
                sself: *mut core::ffi::c_void,
                val: core::ffi::c_int,
            );
        }
        unsafe { vtk_multi_threader_set_global_default_number_of_threads(self.0, val) }
    }
    fn get_global_default_number_of_threads(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_multi_threader_get_global_default_number_of_threads(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_multi_threader_get_global_default_number_of_threads(self.0) }
    }
    fn single_method_execute(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_multi_threader_single_method_execute(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_multi_threader_single_method_execute(self.0) }
    }
    fn multiple_method_execute(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_multi_threader_multiple_method_execute(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_multi_threader_multiple_method_execute(self.0) }
    }
    fn terminate_thread(&mut self, threadId: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_multi_threader_terminate_thread(
                sself: *mut core::ffi::c_void,
                threadId: core::ffi::c_int,
            );
        }
        unsafe { vtk_multi_threader_terminate_thread(self.0, threadId) }
    }
    fn is_thread_active(&mut self, threadId: core::ffi::c_int) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_multi_threader_is_thread_active(
                sself: *mut core::ffi::c_void,
                threadId: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_multi_threader_is_thread_active(self.0, threadId) }
    }
}
impl VtkObject for vtkObject {
    fn is_type_of(&mut self, type_: &str) -> core::ffi::c_int {
        let c_type = std::ffi::CString::new(type_).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_object_is_type_of(
                sself: *mut core::ffi::c_void,
                type_: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_object_is_type_of(self.0, c_type.as_ptr()) }
    }
    fn is_a(&mut self, type_: &str) -> core::ffi::c_int {
        let c_type = std::ffi::CString::new(type_).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_object_is_a(
                sself: *mut core::ffi::c_void,
                type_: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_object_is_a(self.0, c_type.as_ptr()) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_object_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_object_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_object_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_object_new_instance(self.0) }
    }
    fn get_number_of_generations_from_base_type(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong {
        let c_type = std::ffi::CString::new(type_).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_object_get_number_of_generations_from_base_type(
                sself: *mut core::ffi::c_void,
                type_: *const core::ffi::c_char,
            ) -> core::ffi::c_longlong;
        }
        unsafe {
            vtk_object_get_number_of_generations_from_base_type(self.0, c_type.as_ptr())
        }
    }
    fn get_number_of_generations_from_base(
        &mut self,
        type_: &str,
    ) -> core::ffi::c_longlong {
        let c_type = std::ffi::CString::new(type_).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_object_get_number_of_generations_from_base(
                sself: *mut core::ffi::c_void,
                type_: *const core::ffi::c_char,
            ) -> core::ffi::c_longlong;
        }
        unsafe {
            vtk_object_get_number_of_generations_from_base(self.0, c_type.as_ptr())
        }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_object_new(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_object_new(self.0) }
    }
    fn debug_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_object_debug_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_object_debug_on(self.0) }
    }
    fn debug_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_object_debug_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_object_debug_off(self.0) }
    }
    fn get_debug(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_object_get_debug(sself: *mut core::ffi::c_void) -> bool;
        }
        unsafe { vtk_object_get_debug(self.0) }
    }
    fn set_debug(&mut self, debugFlag: bool) -> () {
        unsafe extern "C" {
            fn vtk_object_set_debug(sself: *mut core::ffi::c_void, debugFlag: bool);
        }
        unsafe { vtk_object_set_debug(self.0, debugFlag) }
    }
    fn break_on_error(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_object_break_on_error(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_object_break_on_error(self.0) }
    }
    fn modified(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_object_modified(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_object_modified(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_object_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_object_get_m_time(self.0) }
    }
    fn set_global_warning_display(&mut self, val: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_object_set_global_warning_display(
                sself: *mut core::ffi::c_void,
                val: core::ffi::c_int,
            );
        }
        unsafe { vtk_object_set_global_warning_display(self.0, val) }
    }
    fn global_warning_display_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_object_global_warning_display_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_object_global_warning_display_on(self.0) }
    }
    fn global_warning_display_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_object_global_warning_display_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_object_global_warning_display_off(self.0) }
    }
    fn get_global_warning_display(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_object_get_global_warning_display(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_object_get_global_warning_display(self.0) }
    }
    fn add_observer(
        &mut self,
        event: core::ffi::c_ulong,
        p1: *mut core::ffi::c_void,
        priority: core::ffi::c_float,
    ) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_object_add_observer(
                sself: *mut core::ffi::c_void,
                event: core::ffi::c_ulong,
                p1: *mut core::ffi::c_void,
                priority: core::ffi::c_float,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_object_add_observer(self.0, event, p1, priority) }
    }
    fn get_command(&mut self, tag: core::ffi::c_ulong) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_object_get_command(
                sself: *mut core::ffi::c_void,
                tag: core::ffi::c_ulong,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_object_get_command(self.0, tag) }
    }
    fn remove_observer(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_object_remove_observer(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_object_remove_observer(self.0, p0) }
    }
    fn remove_observers(
        &mut self,
        event: core::ffi::c_ulong,
        p1: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_object_remove_observers(
                sself: *mut core::ffi::c_void,
                event: core::ffi::c_ulong,
                p1: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_object_remove_observers(self.0, event, p1) }
    }
    fn has_observer(
        &mut self,
        event: core::ffi::c_ulong,
        p1: *mut core::ffi::c_void,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_object_has_observer(
                sself: *mut core::ffi::c_void,
                event: core::ffi::c_ulong,
                p1: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_object_has_observer(self.0, event, p1) }
    }
    fn remove_all_observers(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_object_remove_all_observers(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_object_remove_all_observers(self.0) }
    }
}
impl VtkObjectFactoryCollection for vtkObjectFactoryCollection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_object_factory_collection_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_object_factory_collection_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_object_factory_collection_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_object_factory_collection_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_object_factory_collection_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_object_factory_collection_new(self.0) }
    }
    fn add_item(&mut self, t: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_object_factory_collection_add_item(
                sself: *mut core::ffi::c_void,
                t: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_object_factory_collection_add_item(self.0, t) }
    }
    fn get_next_item(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_object_factory_collection_get_next_item(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_object_factory_collection_get_next_item(self.0) }
    }
}
impl VtkOldStyleCallbackCommand for vtkOldStyleCallbackCommand {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_old_style_callback_command_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_old_style_callback_command_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_old_style_callback_command_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_old_style_callback_command_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_old_style_callback_command_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_old_style_callback_command_new(self.0) }
    }
    fn set_callback(&mut self, f: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_old_style_callback_command_set_callback(
                sself: *mut core::ffi::c_void,
                f: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_old_style_callback_command_set_callback(self.0, f) }
    }
    fn set_client_data_delete_callback(&mut self, f: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_old_style_callback_command_set_client_data_delete_callback(
                sself: *mut core::ffi::c_void,
                f: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtk_old_style_callback_command_set_client_data_delete_callback(self.0, f)
        }
    }
}
impl VtkOutputWindow for vtkOutputWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_output_window_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_output_window_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_output_window_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_output_window_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_output_window_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_output_window_new(self.0) }
    }
    fn get_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_output_window_get_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_output_window_get_instance(self.0) }
    }
    fn set_instance(&mut self, instance: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_output_window_set_instance(
                sself: *mut core::ffi::c_void,
                instance: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_output_window_set_instance(self.0, instance) }
    }
    fn display_text(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_output_window_display_text(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_output_window_display_text(self.0, c_p0.as_ptr()) }
    }
    fn display_error_text(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_output_window_display_error_text(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_output_window_display_error_text(self.0, c_p0.as_ptr()) }
    }
    fn display_warning_text(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_output_window_display_warning_text(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_output_window_display_warning_text(self.0, c_p0.as_ptr()) }
    }
    fn display_generic_warning_text(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_output_window_display_generic_warning_text(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_output_window_display_generic_warning_text(self.0, c_p0.as_ptr()) }
    }
    fn display_debug_text(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_output_window_display_debug_text(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_output_window_display_debug_text(self.0, c_p0.as_ptr()) }
    }
    fn prompt_user_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_output_window_prompt_user_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_output_window_prompt_user_on(self.0) }
    }
    fn prompt_user_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_output_window_prompt_user_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_output_window_prompt_user_off(self.0) }
    }
    fn set_prompt_user(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_output_window_set_prompt_user(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_output_window_set_prompt_user(self.0, _arg) }
    }
    fn set_use_std_error_for_all_messages(&mut self, p0: bool) -> () {
        unsafe extern "C" {
            fn vtk_output_window_set_use_std_error_for_all_messages(
                sself: *mut core::ffi::c_void,
                p0: bool,
            );
        }
        unsafe { vtk_output_window_set_use_std_error_for_all_messages(self.0, p0) }
    }
    fn get_use_std_error_for_all_messages(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_output_window_get_use_std_error_for_all_messages(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_output_window_get_use_std_error_for_all_messages(self.0) }
    }
    fn use_std_error_for_all_messages_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_output_window_use_std_error_for_all_messages_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_output_window_use_std_error_for_all_messages_on(self.0) }
    }
    fn use_std_error_for_all_messages_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_output_window_use_std_error_for_all_messages_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_output_window_use_std_error_for_all_messages_off(self.0) }
    }
    fn set_display_mode(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_output_window_set_display_mode(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_output_window_set_display_mode(self.0, _arg) }
    }
    fn get_display_mode_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_output_window_get_display_mode_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_output_window_get_display_mode_min_value(self.0) }
    }
    fn get_display_mode_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_output_window_get_display_mode_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_output_window_get_display_mode_max_value(self.0) }
    }
    fn get_display_mode(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_output_window_get_display_mode(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_output_window_get_display_mode(self.0) }
    }
    fn set_display_mode_to_default(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_output_window_set_display_mode_to_default(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_output_window_set_display_mode_to_default(self.0) }
    }
    fn set_display_mode_to_never(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_output_window_set_display_mode_to_never(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_output_window_set_display_mode_to_never(self.0) }
    }
    fn set_display_mode_to_always(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_output_window_set_display_mode_to_always(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_output_window_set_display_mode_to_always(self.0) }
    }
    fn set_display_mode_to_always_std_err(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_output_window_set_display_mode_to_always_std_err(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_output_window_set_display_mode_to_always_std_err(self.0) }
    }
}
impl VtkOverrideInformationCollection for vtkOverrideInformationCollection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_override_information_collection_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_override_information_collection_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_override_information_collection_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_override_information_collection_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_override_information_collection_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_override_information_collection_new(self.0) }
    }
    fn add_item(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_override_information_collection_add_item(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_override_information_collection_add_item(self.0, p0) }
    }
    fn get_next_item(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_override_information_collection_get_next_item(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_override_information_collection_get_next_item(self.0) }
    }
}
impl VtkPoints for vtkPoints {
    fn new(&mut self, dataType: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_points_new(
                sself: *mut core::ffi::c_void,
                dataType: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_points_new(self.0, dataType) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_points_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_points_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_points_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_points_new_instance(self.0) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_points_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                ext: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_points_allocate(self.0, sz, ext) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_initialize(self.0) }
    }
    fn set_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_set_data(self.0, p0) }
    }
    fn get_data(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_points_get_data(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_points_get_data(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_points_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_points_get_data_type(self.0) }
    }
    fn set_data_type(&mut self, dataType: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type(
                sself: *mut core::ffi::c_void,
                dataType: core::ffi::c_int,
            );
        }
        unsafe { vtk_points_set_data_type(self.0, dataType) }
    }
    fn set_data_type_to_bit(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_bit(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_bit(self.0) }
    }
    fn set_data_type_to_char(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_char(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_char(self.0) }
    }
    fn set_data_type_to_unsigned_char(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_unsigned_char(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_unsigned_char(self.0) }
    }
    fn set_data_type_to_short(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_short(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_short(self.0) }
    }
    fn set_data_type_to_unsigned_short(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_unsigned_short(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_unsigned_short(self.0) }
    }
    fn set_data_type_to_int(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_int(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_int(self.0) }
    }
    fn set_data_type_to_unsigned_int(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_unsigned_int(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_unsigned_int(self.0) }
    }
    fn set_data_type_to_long(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_long(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_long(self.0) }
    }
    fn set_data_type_to_unsigned_long(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_unsigned_long(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_unsigned_long(self.0) }
    }
    fn set_data_type_to_float(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_float(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_float(self.0) }
    }
    fn set_data_type_to_double(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_set_data_type_to_double(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_set_data_type_to_double(self.0) }
    }
    fn squeeze(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_squeeze(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_squeeze(self.0) }
    }
    fn reset(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_reset(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_reset(self.0) }
    }
    fn deep_copy(&mut self, ad: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_points_deep_copy(
                sself: *mut core::ffi::c_void,
                ad: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_deep_copy(self.0, ad) }
    }
    fn shallow_copy(&mut self, ad: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_points_shallow_copy(
                sself: *mut core::ffi::c_void,
                ad: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_shallow_copy(self.0, ad) }
    }
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_points_get_actual_memory_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_points_get_actual_memory_size(self.0) }
    }
    fn get_number_of_points(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_points_get_number_of_points(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_points_get_number_of_points(self.0) }
    }
    fn set_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_points_set_point(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_points_set_point(self.0, id, x, y, z) }
    }
    fn insert_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_points_insert_point(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_points_insert_point(self.0, id, x, y, z) }
    }
    fn insert_points(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_points_insert_points(
                sself: *mut core::ffi::c_void,
                dstIds: *mut core::ffi::c_void,
                srcIds: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_insert_points(self.0, dstIds, srcIds, source) }
    }
    fn insert_next_point(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_points_insert_next_point(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_points_insert_next_point(self.0, x, y, z) }
    }
    fn set_number_of_points(&mut self, numPoints: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_points_set_number_of_points(
                sself: *mut core::ffi::c_void,
                numPoints: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_points_set_number_of_points(self.0, numPoints) }
    }
    fn resize(&mut self, numPoints: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_points_resize(
                sself: *mut core::ffi::c_void,
                numPoints: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_points_resize(self.0, numPoints) }
    }
    fn get_points(
        &mut self,
        ptId: *mut core::ffi::c_void,
        outPoints: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_points_get_points(
                sself: *mut core::ffi::c_void,
                ptId: *mut core::ffi::c_void,
                outPoints: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_get_points(self.0, ptId, outPoints) }
    }
    fn compute_bounds(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_compute_bounds(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_compute_bounds(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_points_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_points_get_m_time(self.0) }
    }
    fn modified(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_modified(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_modified(self.0) }
    }
}
impl VtkPoints2D for vtkPoints2D {
    fn new(&mut self, dataType: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_points_2_d_new(
                sself: *mut core::ffi::c_void,
                dataType: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_points_2_d_new(self.0, dataType) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_points_2_d_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_points_2_d_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_points_2_d_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_points_2_d_new_instance(self.0) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_points_2_d_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                ext: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_points_2_d_allocate(self.0, sz, ext) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_initialize(self.0) }
    }
    fn set_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_2_d_set_data(self.0, p0) }
    }
    fn get_data(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_points_2_d_get_data(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_points_2_d_get_data(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_points_2_d_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_points_2_d_get_data_type(self.0) }
    }
    fn set_data_type(&mut self, dataType: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type(
                sself: *mut core::ffi::c_void,
                dataType: core::ffi::c_int,
            );
        }
        unsafe { vtk_points_2_d_set_data_type(self.0, dataType) }
    }
    fn set_data_type_to_bit(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_bit(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_set_data_type_to_bit(self.0) }
    }
    fn set_data_type_to_char(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_char(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_set_data_type_to_char(self.0) }
    }
    fn set_data_type_to_unsigned_char(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_unsigned_char(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_2_d_set_data_type_to_unsigned_char(self.0) }
    }
    fn set_data_type_to_short(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_short(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_set_data_type_to_short(self.0) }
    }
    fn set_data_type_to_unsigned_short(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_unsigned_short(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_2_d_set_data_type_to_unsigned_short(self.0) }
    }
    fn set_data_type_to_int(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_int(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_set_data_type_to_int(self.0) }
    }
    fn set_data_type_to_unsigned_int(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_unsigned_int(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_2_d_set_data_type_to_unsigned_int(self.0) }
    }
    fn set_data_type_to_long(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_long(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_set_data_type_to_long(self.0) }
    }
    fn set_data_type_to_unsigned_long(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_unsigned_long(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_2_d_set_data_type_to_unsigned_long(self.0) }
    }
    fn set_data_type_to_float(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_float(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_set_data_type_to_float(self.0) }
    }
    fn set_data_type_to_double(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_data_type_to_double(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_set_data_type_to_double(self.0) }
    }
    fn squeeze(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_squeeze(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_squeeze(self.0) }
    }
    fn reset(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_reset(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_reset(self.0) }
    }
    fn deep_copy(&mut self, ad: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_deep_copy(
                sself: *mut core::ffi::c_void,
                ad: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_2_d_deep_copy(self.0, ad) }
    }
    fn shallow_copy(&mut self, ad: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_shallow_copy(
                sself: *mut core::ffi::c_void,
                ad: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_2_d_shallow_copy(self.0, ad) }
    }
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_points_2_d_get_actual_memory_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_points_2_d_get_actual_memory_size(self.0) }
    }
    fn get_number_of_points(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_points_2_d_get_number_of_points(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_points_2_d_get_number_of_points(self.0) }
    }
    fn set_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_point(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
            );
        }
        unsafe { vtk_points_2_d_set_point(self.0, id, x, y) }
    }
    fn insert_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_insert_point(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
            );
        }
        unsafe { vtk_points_2_d_insert_point(self.0, id, x, y) }
    }
    fn insert_next_point(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_points_2_d_insert_next_point(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_points_2_d_insert_next_point(self.0, x, y) }
    }
    fn remove_point(&mut self, id: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_remove_point(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_points_2_d_remove_point(self.0, id) }
    }
    fn set_number_of_points(&mut self, numPoints: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_set_number_of_points(
                sself: *mut core::ffi::c_void,
                numPoints: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_points_2_d_set_number_of_points(self.0, numPoints) }
    }
    fn resize(&mut self, numPoints: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_points_2_d_resize(
                sself: *mut core::ffi::c_void,
                numPoints: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_points_2_d_resize(self.0, numPoints) }
    }
    fn get_points(
        &mut self,
        ptId: *mut core::ffi::c_void,
        fp: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_get_points(
                sself: *mut core::ffi::c_void,
                ptId: *mut core::ffi::c_void,
                fp: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_points_2_d_get_points(self.0, ptId, fp) }
    }
    fn compute_bounds(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_points_2_d_compute_bounds(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_points_2_d_compute_bounds(self.0) }
    }
}
impl VtkPriorityQueue for vtkPriorityQueue {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_priority_queue_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_priority_queue_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_priority_queue_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_priority_queue_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_priority_queue_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_priority_queue_new_instance(self.0) }
    }
    fn allocate(&mut self, sz: core::ffi::c_longlong, ext: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_priority_queue_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                ext: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_priority_queue_allocate(self.0, sz, ext) }
    }
    fn insert(
        &mut self,
        priority: core::ffi::c_double,
        id: core::ffi::c_longlong,
    ) -> () {
        unsafe extern "C" {
            fn vtk_priority_queue_insert(
                sself: *mut core::ffi::c_void,
                priority: core::ffi::c_double,
                id: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_priority_queue_insert(self.0, priority, id) }
    }
    fn pop(
        &mut self,
        location: core::ffi::c_longlong,
        priority: &mut core::ffi::c_double,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_priority_queue_pop(
                sself: *mut core::ffi::c_void,
                location: core::ffi::c_longlong,
                priority: &mut core::ffi::c_double,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_priority_queue_pop(self.0, location, priority) }
    }
    fn peek(
        &mut self,
        location: core::ffi::c_longlong,
        priority: &mut core::ffi::c_double,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_priority_queue_peek(
                sself: *mut core::ffi::c_void,
                location: core::ffi::c_longlong,
                priority: &mut core::ffi::c_double,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_priority_queue_peek(self.0, location, priority) }
    }
    fn delete_id(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_priority_queue_delete_id(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_priority_queue_delete_id(self.0, id) }
    }
    fn get_priority(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_priority_queue_get_priority(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_priority_queue_get_priority(self.0, id) }
    }
    fn get_number_of_items(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_priority_queue_get_number_of_items(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_priority_queue_get_number_of_items(self.0) }
    }
    fn reset(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_priority_queue_reset(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_priority_queue_reset(self.0) }
    }
}
impl VtkRandomPool for vtkRandomPool {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_random_pool_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_random_pool_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_random_pool_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_random_pool_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_random_pool_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_random_pool_new_instance(self.0) }
    }
    fn set_sequence(&mut self, seq: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_random_pool_set_sequence(
                sself: *mut core::ffi::c_void,
                seq: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_random_pool_set_sequence(self.0, seq) }
    }
    fn get_sequence(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_random_pool_get_sequence(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_random_pool_get_sequence(self.0) }
    }
    fn set_size(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_random_pool_set_size(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_random_pool_set_size(self.0, _arg) }
    }
    fn get_size_min_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_size_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_size_min_value(self.0) }
    }
    fn get_size_max_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_size_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_size_max_value(self.0) }
    }
    fn get_size(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_size(self.0) }
    }
    fn set_number_of_components(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_random_pool_set_number_of_components(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_random_pool_set_number_of_components(self.0, _arg) }
    }
    fn get_number_of_components_min_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_number_of_components_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_number_of_components_min_value(self.0) }
    }
    fn get_number_of_components_max_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_number_of_components_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_number_of_components_max_value(self.0) }
    }
    fn get_number_of_components(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_number_of_components(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_number_of_components(self.0) }
    }
    fn get_total_size(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_total_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_total_size(self.0) }
    }
    fn get_value(&mut self, i: core::ffi::c_longlong) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_random_pool_get_value(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_random_pool_get_value(self.0, i) }
    }
    fn populate_data_array(
        &mut self,
        da: *mut core::ffi::c_void,
        minRange: core::ffi::c_double,
        maxRange: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_random_pool_populate_data_array(
                sself: *mut core::ffi::c_void,
                da: *mut core::ffi::c_void,
                minRange: core::ffi::c_double,
                maxRange: core::ffi::c_double,
            );
        }
        unsafe { vtk_random_pool_populate_data_array(self.0, da, minRange, maxRange) }
    }
    fn set_chunk_size(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_random_pool_set_chunk_size(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_random_pool_set_chunk_size(self.0, _arg) }
    }
    fn get_chunk_size_min_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_chunk_size_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_chunk_size_min_value(self.0) }
    }
    fn get_chunk_size_max_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_chunk_size_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_chunk_size_max_value(self.0) }
    }
    fn get_chunk_size(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_pool_get_chunk_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_pool_get_chunk_size(self.0) }
    }
}
impl VtkReferenceCount for vtkReferenceCount {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_reference_count_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_reference_count_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_reference_count_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_reference_count_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_reference_count_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_reference_count_new_instance(self.0) }
    }
}
impl VtkScalarsToColors for vtkScalarsToColors {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_scalars_to_colors_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_scalars_to_colors_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_scalars_to_colors_new(self.0) }
    }
    fn is_opaque(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_is_opaque(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_scalars_to_colors_is_opaque(self.0) }
    }
    fn build(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_build(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_scalars_to_colors_build(self.0) }
    }
    fn set_range(&mut self, min: core::ffi::c_double, max: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_range(
                sself: *mut core::ffi::c_void,
                min: core::ffi::c_double,
                max: core::ffi::c_double,
            );
        }
        unsafe { vtk_scalars_to_colors_set_range(self.0, min, max) }
    }
    fn get_opacity(&mut self, v: core::ffi::c_double) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_opacity(
                sself: *mut core::ffi::c_void,
                v: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_scalars_to_colors_get_opacity(self.0, v) }
    }
    fn get_luminance(&mut self, x: core::ffi::c_double) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_luminance(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_scalars_to_colors_get_luminance(self.0, x) }
    }
    fn set_alpha(&mut self, alpha: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_alpha(
                sself: *mut core::ffi::c_void,
                alpha: core::ffi::c_double,
            );
        }
        unsafe { vtk_scalars_to_colors_set_alpha(self.0, alpha) }
    }
    fn get_alpha(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_alpha(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_scalars_to_colors_get_alpha(self.0) }
    }
    fn map_scalars(
        &mut self,
        scalars: *mut core::ffi::c_void,
        colorMode: core::ffi::c_int,
        component: core::ffi::c_int,
        outputFormat: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_map_scalars(
                sself: *mut core::ffi::c_void,
                scalars: *mut core::ffi::c_void,
                colorMode: core::ffi::c_int,
                component: core::ffi::c_int,
                outputFormat: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_scalars_to_colors_map_scalars(
                self.0,
                scalars,
                colorMode,
                component,
                outputFormat,
            )
        }
    }
    fn set_vector_mode(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_vector_mode(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_scalars_to_colors_set_vector_mode(self.0, _arg) }
    }
    fn get_vector_mode(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_vector_mode(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_scalars_to_colors_get_vector_mode(self.0) }
    }
    fn set_vector_mode_to_magnitude(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_vector_mode_to_magnitude(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_scalars_to_colors_set_vector_mode_to_magnitude(self.0) }
    }
    fn set_vector_mode_to_component(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_vector_mode_to_component(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_scalars_to_colors_set_vector_mode_to_component(self.0) }
    }
    fn set_vector_mode_to_rgb_colors(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_vector_mode_to_rgb_colors(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_scalars_to_colors_set_vector_mode_to_rgb_colors(self.0) }
    }
    fn set_vector_component(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_vector_component(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_scalars_to_colors_set_vector_component(self.0, _arg) }
    }
    fn get_vector_component(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_vector_component(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_scalars_to_colors_get_vector_component(self.0) }
    }
    fn set_vector_size(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_vector_size(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_scalars_to_colors_set_vector_size(self.0, _arg) }
    }
    fn get_vector_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_vector_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_scalars_to_colors_get_vector_size(self.0) }
    }
    fn deep_copy(&mut self, o: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_deep_copy(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_scalars_to_colors_deep_copy(self.0, o) }
    }
    fn using_log_scale(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_using_log_scale(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_scalars_to_colors_using_log_scale(self.0) }
    }
    fn get_number_of_available_colors(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_number_of_available_colors(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_scalars_to_colors_get_number_of_available_colors(self.0) }
    }
    fn set_annotations(
        &mut self,
        values: *mut core::ffi::c_void,
        annotations: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_annotations(
                sself: *mut core::ffi::c_void,
                values: *mut core::ffi::c_void,
                annotations: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_scalars_to_colors_set_annotations(self.0, values, annotations) }
    }
    fn get_annotated_values(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_annotated_values(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_scalars_to_colors_get_annotated_values(self.0) }
    }
    fn get_annotations(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_annotations(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_scalars_to_colors_get_annotations(self.0) }
    }
    fn get_number_of_annotated_values(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_number_of_annotated_values(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_scalars_to_colors_get_number_of_annotated_values(self.0) }
    }
    fn reset_annotations(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_reset_annotations(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_scalars_to_colors_reset_annotations(self.0) }
    }
    fn set_indexed_lookup(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_set_indexed_lookup(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_scalars_to_colors_set_indexed_lookup(self.0, _arg) }
    }
    fn get_indexed_lookup(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_get_indexed_lookup(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_scalars_to_colors_get_indexed_lookup(self.0) }
    }
    fn indexed_lookup_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_indexed_lookup_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_scalars_to_colors_indexed_lookup_on(self.0) }
    }
    fn indexed_lookup_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_scalars_to_colors_indexed_lookup_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_scalars_to_colors_indexed_lookup_off(self.0) }
    }
}
impl VtkShortArray for vtkShortArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_short_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_short_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_short_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_short_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_short_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_short_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_short_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_short_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_short_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_short_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_short {
        unsafe extern "C" {
            fn vtk_short_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_short;
        }
        unsafe { vtk_short_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_short) -> () {
        unsafe extern "C" {
            fn vtk_short_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_short,
            );
        }
        unsafe { vtk_short_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_short_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_short_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_short) -> () {
        unsafe extern "C" {
            fn vtk_short_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_short,
            );
        }
        unsafe { vtk_short_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_short) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_short_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_short,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_short_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_short_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_short_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_short {
        unsafe extern "C" {
            fn vtk_short_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_short;
        }
        unsafe { vtk_short_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_short {
        unsafe extern "C" {
            fn vtk_short_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_short;
        }
        unsafe { vtk_short_array_get_data_type_value_max(self.0) }
    }
}
impl VtkSignedCharArray for vtkSignedCharArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_signed_char_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_signed_char_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_signed_char_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_signed_char_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_signed_char_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_signed_char_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_signed_char_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_signed_char_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_signed_char_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_signed_char_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_schar {
        unsafe extern "C" {
            fn vtk_signed_char_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_schar;
        }
        unsafe { vtk_signed_char_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_schar) -> () {
        unsafe extern "C" {
            fn vtk_signed_char_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_schar,
            );
        }
        unsafe { vtk_signed_char_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_signed_char_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_signed_char_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_schar) -> () {
        unsafe extern "C" {
            fn vtk_signed_char_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_schar,
            );
        }
        unsafe { vtk_signed_char_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_schar) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_signed_char_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_schar,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_signed_char_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_signed_char_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_signed_char_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_schar {
        unsafe extern "C" {
            fn vtk_signed_char_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_schar;
        }
        unsafe { vtk_signed_char_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_schar {
        unsafe extern "C" {
            fn vtk_signed_char_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_schar;
        }
        unsafe { vtk_signed_char_array_get_data_type_value_max(self.0) }
    }
}
impl VtkSortDataArray for vtkSortDataArray {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sort_data_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sort_data_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sort_data_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sort_data_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sort_data_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sort_data_array_new_instance(self.0) }
    }
    fn sort(&mut self, keys: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_sort_data_array_sort(
                sself: *mut core::ffi::c_void,
                keys: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_sort_data_array_sort(self.0, keys) }
    }
    fn sort_array_by_component(
        &mut self,
        arr: *mut core::ffi::c_void,
        k: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_sort_data_array_sort_array_by_component(
                sself: *mut core::ffi::c_void,
                arr: *mut core::ffi::c_void,
                k: core::ffi::c_int,
            );
        }
        unsafe { vtk_sort_data_array_sort_array_by_component(self.0, arr, k) }
    }
}
impl VtkStringArray for vtkStringArray {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_array_extended_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_array_new_instance(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_string_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_string_array_get_data_type(self.0) }
    }
    fn is_numeric(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_string_array_is_numeric(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_string_array_is_numeric(self.0) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_string_array_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_string_array_initialize(self.0) }
    }
    fn get_data_type_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_string_array_get_data_type_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_string_array_get_data_type_size(self.0) }
    }
    fn squeeze(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_string_array_squeeze(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_string_array_squeeze(self.0) }
    }
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_string_array_resize(
                sself: *mut core::ffi::c_void,
                numTuples: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_string_array_resize(self.0, numTuples) }
    }
    fn set_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_string_array_set_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_string_array_set_tuple(self.0, i, j, source) }
    }
    fn insert_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_string_array_insert_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_string_array_insert_tuple(self.0, i, j, source) }
    }
    fn insert_tuples(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_string_array_insert_tuples(
                sself: *mut core::ffi::c_void,
                dstIds: *mut core::ffi::c_void,
                srcIds: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_string_array_insert_tuples(self.0, dstIds, srcIds, source) }
    }
    fn insert_next_tuple(
        &mut self,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_string_array_insert_next_tuple(
                sself: *mut core::ffi::c_void,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_string_array_insert_next_tuple(self.0, j, source) }
    }
    fn get_tuples(
        &mut self,
        ptIds: *mut core::ffi::c_void,
        output: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_string_array_get_tuples(
                sself: *mut core::ffi::c_void,
                ptIds: *mut core::ffi::c_void,
                output: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_string_array_get_tuples(self.0, ptIds, output) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_string_array_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                ext: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_string_array_allocate(self.0, sz, ext) }
    }
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_string_array_set_number_of_tuples(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_string_array_set_number_of_tuples(self.0, number) }
    }
    fn get_number_of_values(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_string_array_get_number_of_values(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_string_array_get_number_of_values(self.0) }
    }
    fn get_number_of_element_components(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_string_array_get_number_of_element_components(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_string_array_get_number_of_element_components(self.0) }
    }
    fn get_element_component_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_string_array_get_element_component_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_string_array_get_element_component_size(self.0) }
    }
    fn write_pointer(
        &mut self,
        id: core::ffi::c_longlong,
        number: core::ffi::c_longlong,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_array_write_pointer(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                number: core::ffi::c_longlong,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_array_write_pointer(self.0, id, number) }
    }
    fn get_pointer(&mut self, id: core::ffi::c_longlong) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_array_get_pointer(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_array_get_pointer(self.0, id) }
    }
    fn deep_copy(&mut self, aa: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_string_array_deep_copy(
                sself: *mut core::ffi::c_void,
                aa: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_string_array_deep_copy(self.0, aa) }
    }
    fn set_array(
        &mut self,
        array: *mut core::ffi::c_void,
        size: core::ffi::c_longlong,
        save: core::ffi::c_int,
        deleteMethod: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_string_array_set_array(
                sself: *mut core::ffi::c_void,
                array: *mut core::ffi::c_void,
                size: core::ffi::c_longlong,
                save: core::ffi::c_int,
                deleteMethod: core::ffi::c_int,
            );
        }
        unsafe { vtk_string_array_set_array(self.0, array, size, save, deleteMethod) }
    }
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_string_array_set_array_free_function(
                sself: *mut core::ffi::c_void,
                callback: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_string_array_set_array_free_function(self.0, callback) }
    }
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_string_array_get_actual_memory_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_string_array_get_actual_memory_size(self.0) }
    }
    fn new_iterator(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_array_new_iterator(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_array_new_iterator(self.0) }
    }
    fn get_data_size(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_string_array_get_data_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_string_array_get_data_size(self.0) }
    }
    fn data_changed(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_string_array_data_changed(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_string_array_data_changed(self.0) }
    }
    fn data_element_changed(&mut self, id: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_string_array_data_element_changed(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_string_array_data_element_changed(self.0, id) }
    }
    fn clear_lookup(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_string_array_clear_lookup(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_string_array_clear_lookup(self.0) }
    }
}
impl VtkStringOutputWindow for vtkStringOutputWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_output_window_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_output_window_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_output_window_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_output_window_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_string_output_window_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_string_output_window_new(self.0) }
    }
    fn display_text(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_string_output_window_display_text(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_string_output_window_display_text(self.0, c_p0.as_ptr()) }
    }
}
impl VtkTimePointUtility for vtkTimePointUtility {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_time_point_utility_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_time_point_utility_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_time_point_utility_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_time_point_utility_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_time_point_utility_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_time_point_utility_new_instance(self.0) }
    }
    fn date_to_time_point(
        &mut self,
        year: core::ffi::c_int,
        month: core::ffi::c_int,
        day: core::ffi::c_int,
    ) -> core::ffi::c_ulonglong {
        unsafe extern "C" {
            fn vtk_time_point_utility_date_to_time_point(
                sself: *mut core::ffi::c_void,
                year: core::ffi::c_int,
                month: core::ffi::c_int,
                day: core::ffi::c_int,
            ) -> core::ffi::c_ulonglong;
        }
        unsafe { vtk_time_point_utility_date_to_time_point(self.0, year, month, day) }
    }
    fn time_to_time_point(
        &mut self,
        hour: core::ffi::c_int,
        minute: core::ffi::c_int,
        second: core::ffi::c_int,
        millis: core::ffi::c_int,
    ) -> core::ffi::c_ulonglong {
        unsafe extern "C" {
            fn vtk_time_point_utility_time_to_time_point(
                sself: *mut core::ffi::c_void,
                hour: core::ffi::c_int,
                minute: core::ffi::c_int,
                second: core::ffi::c_int,
                millis: core::ffi::c_int,
            ) -> core::ffi::c_ulonglong;
        }
        unsafe {
            vtk_time_point_utility_time_to_time_point(
                self.0,
                hour,
                minute,
                second,
                millis,
            )
        }
    }
    fn date_time_to_time_point(
        &mut self,
        year: core::ffi::c_int,
        month: core::ffi::c_int,
        day: core::ffi::c_int,
        hour: core::ffi::c_int,
        minute: core::ffi::c_int,
        sec: core::ffi::c_int,
        millis: core::ffi::c_int,
    ) -> core::ffi::c_ulonglong {
        unsafe extern "C" {
            fn vtk_time_point_utility_date_time_to_time_point(
                sself: *mut core::ffi::c_void,
                year: core::ffi::c_int,
                month: core::ffi::c_int,
                day: core::ffi::c_int,
                hour: core::ffi::c_int,
                minute: core::ffi::c_int,
                sec: core::ffi::c_int,
                millis: core::ffi::c_int,
            ) -> core::ffi::c_ulonglong;
        }
        unsafe {
            vtk_time_point_utility_date_time_to_time_point(
                self.0,
                year,
                month,
                day,
                hour,
                minute,
                sec,
                millis,
            )
        }
    }
    fn get_date(
        &mut self,
        time: core::ffi::c_ulonglong,
        year: &mut core::ffi::c_int,
        month: &mut core::ffi::c_int,
        day: &mut core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_date(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
                year: &mut core::ffi::c_int,
                month: &mut core::ffi::c_int,
                day: &mut core::ffi::c_int,
            );
        }
        unsafe { vtk_time_point_utility_get_date(self.0, time, year, month, day) }
    }
    fn get_time(
        &mut self,
        time: core::ffi::c_ulonglong,
        hour: &mut core::ffi::c_int,
        minute: &mut core::ffi::c_int,
        second: &mut core::ffi::c_int,
        millis: &mut core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_time(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
                hour: &mut core::ffi::c_int,
                minute: &mut core::ffi::c_int,
                second: &mut core::ffi::c_int,
                millis: &mut core::ffi::c_int,
            );
        }
        unsafe {
            vtk_time_point_utility_get_time(self.0, time, hour, minute, second, millis)
        }
    }
    fn get_date_time(
        &mut self,
        time: core::ffi::c_ulonglong,
        year: &mut core::ffi::c_int,
        month: &mut core::ffi::c_int,
        day: &mut core::ffi::c_int,
        hour: &mut core::ffi::c_int,
        minute: &mut core::ffi::c_int,
        second: &mut core::ffi::c_int,
        millis: &mut core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_date_time(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
                year: &mut core::ffi::c_int,
                month: &mut core::ffi::c_int,
                day: &mut core::ffi::c_int,
                hour: &mut core::ffi::c_int,
                minute: &mut core::ffi::c_int,
                second: &mut core::ffi::c_int,
                millis: &mut core::ffi::c_int,
            );
        }
        unsafe {
            vtk_time_point_utility_get_date_time(
                self.0,
                time,
                year,
                month,
                day,
                hour,
                minute,
                second,
                millis,
            )
        }
    }
    fn get_year(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_year(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_time_point_utility_get_year(self.0, time) }
    }
    fn get_month(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_month(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_time_point_utility_get_month(self.0, time) }
    }
    fn get_day(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_day(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_time_point_utility_get_day(self.0, time) }
    }
    fn get_hour(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_hour(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_time_point_utility_get_hour(self.0, time) }
    }
    fn get_minute(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_minute(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_time_point_utility_get_minute(self.0, time) }
    }
    fn get_second(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_second(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_time_point_utility_get_second(self.0, time) }
    }
    fn get_millisecond(&mut self, time: core::ffi::c_ulonglong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_time_point_utility_get_millisecond(
                sself: *mut core::ffi::c_void,
                time: core::ffi::c_ulonglong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_time_point_utility_get_millisecond(self.0, time) }
    }
    fn time_point_to_iso_8601(
        &mut self,
        p0: core::ffi::c_ulonglong,
        format: core::ffi::c_int,
    ) -> &str {
        unsafe extern "C" {
            fn vtk_time_point_utility_time_point_to_iso_8601(
                sself: *mut core::ffi::c_void,
                p0: core::ffi::c_ulonglong,
                format: core::ffi::c_int,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe {
            vtk_time_point_utility_time_point_to_iso_8601(self.0, p0, format)
        };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}
impl VtkTypeFloat32Array for vtkTypeFloat32Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_float_32_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_float_32_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_float_32_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_float_32_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_float_32_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_float_32_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_float_32_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_float_32_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeFloat64Array for vtkTypeFloat64Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_float_64_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_float_64_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_float_64_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_float_64_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_float_64_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_float_64_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_float_64_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_float_64_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeInt16Array for vtkTypeInt16Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_16_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_16_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_16_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_16_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_16_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_16_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_16_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_16_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeInt32Array for vtkTypeInt32Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_32_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_32_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_32_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_32_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_32_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_32_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_32_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_32_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeInt64Array for vtkTypeInt64Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_64_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_64_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_64_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_64_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_64_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_64_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_64_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_64_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeInt8Array for vtkTypeInt8Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_8_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_8_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_8_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_8_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_8_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_8_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_int_8_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_int_8_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeUInt16Array for vtkTypeUInt16Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_16_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_16_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_16_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_16_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_16_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_16_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_16_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_16_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeUInt32Array for vtkTypeUInt32Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_32_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_32_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_32_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_32_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_32_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_32_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_32_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_32_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeUInt64Array for vtkTypeUInt64Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_64_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_64_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_64_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_64_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_64_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_64_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_64_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_64_array_fast_down_cast(self.0, source) }
    }
}
impl VtkTypeUInt8Array for vtkTypeUInt8Array {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_8_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_8_array_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_8_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_8_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_8_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_8_array_new_instance(self.0) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_type_u_int_8_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_type_u_int_8_array_fast_down_cast(self.0, source) }
    }
}
impl VtkUnicodeStringArray for vtkUnicodeStringArray {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unicode_string_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unicode_string_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unicode_string_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unicode_string_array_extended_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unicode_string_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unicode_string_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unicode_string_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unicode_string_array_new_instance(self.0) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unicode_string_array_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                ext: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unicode_string_array_allocate(self.0, sz, ext) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_unicode_string_array_initialize(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unicode_string_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unicode_string_array_get_data_type(self.0) }
    }
    fn get_data_type_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unicode_string_array_get_data_type_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unicode_string_array_get_data_type_size(self.0) }
    }
    fn get_element_component_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unicode_string_array_get_element_component_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unicode_string_array_get_element_component_size(self.0) }
    }
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_set_number_of_tuples(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_unicode_string_array_set_number_of_tuples(self.0, number) }
    }
    fn set_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_set_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unicode_string_array_set_tuple(self.0, i, j, source) }
    }
    fn insert_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_insert_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unicode_string_array_insert_tuple(self.0, i, j, source) }
    }
    fn insert_tuples(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_insert_tuples(
                sself: *mut core::ffi::c_void,
                dstIds: *mut core::ffi::c_void,
                srcIds: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unicode_string_array_insert_tuples(self.0, dstIds, srcIds, source) }
    }
    fn insert_next_tuple(
        &mut self,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_unicode_string_array_insert_next_tuple(
                sself: *mut core::ffi::c_void,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_unicode_string_array_insert_next_tuple(self.0, j, source) }
    }
    fn squeeze(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_squeeze(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_unicode_string_array_squeeze(self.0) }
    }
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unicode_string_array_resize(
                sself: *mut core::ffi::c_void,
                numTuples: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unicode_string_array_resize(self.0, numTuples) }
    }
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_set_array_free_function(
                sself: *mut core::ffi::c_void,
                callback: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unicode_string_array_set_array_free_function(self.0, callback) }
    }
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_unicode_string_array_get_actual_memory_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_unicode_string_array_get_actual_memory_size(self.0) }
    }
    fn is_numeric(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unicode_string_array_is_numeric(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unicode_string_array_is_numeric(self.0) }
    }
    fn new_iterator(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unicode_string_array_new_iterator(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unicode_string_array_new_iterator(self.0) }
    }
    fn data_changed(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_data_changed(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_unicode_string_array_data_changed(self.0) }
    }
    fn clear_lookup(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_unicode_string_array_clear_lookup(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_unicode_string_array_clear_lookup(self.0) }
    }
    fn insert_next_utf_8_value(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_unicode_string_array_insert_next_utf_8_value(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe {
            vtk_unicode_string_array_insert_next_utf_8_value(self.0, c_p0.as_ptr())
        }
    }
    fn set_utf_8_value(&mut self, i: core::ffi::c_longlong, p1: &str) -> () {
        let c_p1 = std::ffi::CString::new(p1).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_unicode_string_array_set_utf_8_value(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                p1: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_unicode_string_array_set_utf_8_value(self.0, i, c_p1.as_ptr()) }
    }
    fn get_utf_8_value(&mut self, i: core::ffi::c_longlong) -> &str {
        unsafe extern "C" {
            fn vtk_unicode_string_array_get_utf_8_value(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_unicode_string_array_get_utf_8_value(self.0, i) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}
impl VtkUnsignedCharArray for vtkUnsignedCharArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_char_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_char_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_char_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_char_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unsigned_char_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_uchar {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_uchar;
        }
        unsafe { vtk_unsigned_char_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_uchar) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_uchar,
            );
        }
        unsafe { vtk_unsigned_char_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_unsigned_char_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_uchar) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_uchar,
            );
        }
        unsafe { vtk_unsigned_char_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_uchar) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_uchar,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_unsigned_char_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_char_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_uchar {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uchar;
        }
        unsafe { vtk_unsigned_char_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_uchar {
        unsafe extern "C" {
            fn vtk_unsigned_char_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uchar;
        }
        unsafe { vtk_unsigned_char_array_get_data_type_value_max(self.0) }
    }
}
impl VtkUnsignedIntArray for vtkUnsignedIntArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_int_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_int_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_int_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_int_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unsigned_int_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_unsigned_int_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_uint,
            );
        }
        unsafe { vtk_unsigned_int_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_unsigned_int_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_uint,
            );
        }
        unsafe { vtk_unsigned_int_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_uint) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_uint,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_unsigned_int_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_int_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_unsigned_int_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_unsigned_int_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_unsigned_int_array_get_data_type_value_max(self.0) }
    }
}
impl VtkUnsignedLongArray for vtkUnsignedLongArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unsigned_long_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_unsigned_long_array_get_value(self.0, id) }
    }
    fn set_value(&mut self, id: core::ffi::c_longlong, value: core::ffi::c_ulong) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_ulong,
            );
        }
        unsafe { vtk_unsigned_long_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_unsigned_long_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_ulong) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_ulong,
            );
        }
        unsafe { vtk_unsigned_long_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_ulong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_ulong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_unsigned_long_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_unsigned_long_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_unsigned_long_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_unsigned_long_array_get_data_type_value_max(self.0) }
    }
}
impl VtkUnsignedLongLongArray for vtkUnsignedLongLongArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_long_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_long_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_long_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_long_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unsigned_long_long_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_ulonglong {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_ulonglong;
        }
        unsafe { vtk_unsigned_long_long_array_get_value(self.0, id) }
    }
    fn set_value(
        &mut self,
        id: core::ffi::c_longlong,
        value: core::ffi::c_ulonglong,
    ) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_ulonglong,
            );
        }
        unsafe { vtk_unsigned_long_long_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_unsigned_long_long_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(
        &mut self,
        id: core::ffi::c_longlong,
        f: core::ffi::c_ulonglong,
    ) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_ulonglong,
            );
        }
        unsafe { vtk_unsigned_long_long_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_ulonglong) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_ulonglong,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_unsigned_long_long_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_long_long_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_ulonglong {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulonglong;
        }
        unsafe { vtk_unsigned_long_long_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_ulonglong {
        unsafe extern "C" {
            fn vtk_unsigned_long_long_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulonglong;
        }
        unsafe { vtk_unsigned_long_long_array_get_data_type_value_max(self.0) }
    }
}
impl VtkUnsignedShortArray for vtkUnsignedShortArray {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_short_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_short_array_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_short_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_short_array_extended_new(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_unsigned_short_array_get_data_type(self.0) }
    }
    fn get_value(&mut self, id: core::ffi::c_longlong) -> core::ffi::c_ushort {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_get_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> core::ffi::c_ushort;
        }
        unsafe { vtk_unsigned_short_array_get_value(self.0, id) }
    }
    fn set_value(
        &mut self,
        id: core::ffi::c_longlong,
        value: core::ffi::c_ushort,
    ) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_set_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                value: core::ffi::c_ushort,
            );
        }
        unsafe { vtk_unsigned_short_array_set_value(self.0, id, value) }
    }
    fn set_number_of_values(&mut self, number: core::ffi::c_longlong) -> bool {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_set_number_of_values(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            ) -> bool;
        }
        unsafe { vtk_unsigned_short_array_set_number_of_values(self.0, number) }
    }
    fn insert_value(&mut self, id: core::ffi::c_longlong, f: core::ffi::c_ushort) -> () {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_insert_value(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                f: core::ffi::c_ushort,
            );
        }
        unsafe { vtk_unsigned_short_array_insert_value(self.0, id, f) }
    }
    fn insert_next_value(&mut self, f: core::ffi::c_ushort) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_insert_next_value(
                sself: *mut core::ffi::c_void,
                f: core::ffi::c_ushort,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_unsigned_short_array_insert_next_value(self.0, f) }
    }
    fn fast_down_cast(
        &mut self,
        source: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_fast_down_cast(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unsigned_short_array_fast_down_cast(self.0, source) }
    }
    fn get_data_type_value_min(&mut self) -> core::ffi::c_ushort {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_get_data_type_value_min(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ushort;
        }
        unsafe { vtk_unsigned_short_array_get_data_type_value_min(self.0) }
    }
    fn get_data_type_value_max(&mut self) -> core::ffi::c_ushort {
        unsafe extern "C" {
            fn vtk_unsigned_short_array_get_data_type_value_max(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ushort;
        }
        unsafe { vtk_unsigned_short_array_get_data_type_value_max(self.0) }
    }
}
impl VtkVariantArray for vtkVariantArray {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_variant_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_variant_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_variant_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_variant_array_extended_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_variant_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_variant_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_variant_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_variant_array_new_instance(self.0) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_variant_array_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                ext: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_variant_array_allocate(self.0, sz, ext) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_variant_array_initialize(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_variant_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_variant_array_get_data_type(self.0) }
    }
    fn get_data_type_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_variant_array_get_data_type_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_variant_array_get_data_type_size(self.0) }
    }
    fn get_element_component_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_variant_array_get_element_component_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_variant_array_get_element_component_size(self.0) }
    }
    fn set_number_of_tuples(&mut self, number: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_set_number_of_tuples(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_variant_array_set_number_of_tuples(self.0, number) }
    }
    fn set_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_set_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_variant_array_set_tuple(self.0, i, j, source) }
    }
    fn insert_tuple(
        &mut self,
        i: core::ffi::c_longlong,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_insert_tuple(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_longlong,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_variant_array_insert_tuple(self.0, i, j, source) }
    }
    fn insert_tuples(
        &mut self,
        dstIds: *mut core::ffi::c_void,
        srcIds: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_insert_tuples(
                sself: *mut core::ffi::c_void,
                dstIds: *mut core::ffi::c_void,
                srcIds: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_variant_array_insert_tuples(self.0, dstIds, srcIds, source) }
    }
    fn insert_next_tuple(
        &mut self,
        j: core::ffi::c_longlong,
        source: *mut core::ffi::c_void,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_variant_array_insert_next_tuple(
                sself: *mut core::ffi::c_void,
                j: core::ffi::c_longlong,
                source: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_variant_array_insert_next_tuple(self.0, j, source) }
    }
    fn deep_copy(&mut self, da: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_deep_copy(
                sself: *mut core::ffi::c_void,
                da: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_variant_array_deep_copy(self.0, da) }
    }
    fn squeeze(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_squeeze(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_variant_array_squeeze(self.0) }
    }
    fn resize(&mut self, numTuples: core::ffi::c_longlong) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_variant_array_resize(
                sself: *mut core::ffi::c_void,
                numTuples: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_variant_array_resize(self.0, numTuples) }
    }
    fn get_actual_memory_size(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_variant_array_get_actual_memory_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_variant_array_get_actual_memory_size(self.0) }
    }
    fn is_numeric(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_variant_array_is_numeric(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_variant_array_is_numeric(self.0) }
    }
    fn new_iterator(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_variant_array_new_iterator(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_variant_array_new_iterator(self.0) }
    }
    fn get_pointer(&mut self, id: core::ffi::c_longlong) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_variant_array_get_pointer(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_variant_array_get_pointer(self.0, id) }
    }
    fn set_array(
        &mut self,
        arr: *mut core::ffi::c_void,
        size: core::ffi::c_longlong,
        save: core::ffi::c_int,
        deleteMethod: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_set_array(
                sself: *mut core::ffi::c_void,
                arr: *mut core::ffi::c_void,
                size: core::ffi::c_longlong,
                save: core::ffi::c_int,
                deleteMethod: core::ffi::c_int,
            );
        }
        unsafe { vtk_variant_array_set_array(self.0, arr, size, save, deleteMethod) }
    }
    fn set_array_free_function(&mut self, callback: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_set_array_free_function(
                sself: *mut core::ffi::c_void,
                callback: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_variant_array_set_array_free_function(self.0, callback) }
    }
    fn get_number_of_values(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_variant_array_get_number_of_values(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_variant_array_get_number_of_values(self.0) }
    }
    fn data_changed(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_data_changed(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_variant_array_data_changed(self.0) }
    }
    fn data_element_changed(&mut self, id: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_data_element_changed(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_variant_array_data_element_changed(self.0, id) }
    }
    fn clear_lookup(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_variant_array_clear_lookup(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_variant_array_clear_lookup(self.0) }
    }
}
impl VtkVersion for vtkVersion {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_version_new(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_version_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_version_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_version_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_version_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_version_new_instance(self.0) }
    }
    fn get_vtk_version(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_version_get_vtk_version(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_version_get_vtk_version(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn get_vtk_version_full(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_version_get_vtk_version_full(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_version_get_vtk_version_full(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn get_vtk_major_version(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_version_get_vtk_major_version(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_version_get_vtk_major_version(self.0) }
    }
    fn get_vtk_minor_version(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_version_get_vtk_minor_version(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_version_get_vtk_minor_version(self.0) }
    }
    fn get_vtk_build_version(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_version_get_vtk_build_version(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_version_get_vtk_build_version(self.0) }
    }
    fn get_vtk_source_version(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_version_get_vtk_source_version(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_version_get_vtk_source_version(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}
impl VtkVoidArray for vtkVoidArray {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_void_array_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_void_array_new(self.0) }
    }
    fn extended_new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_void_array_extended_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_void_array_extended_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_void_array_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_void_array_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_void_array_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_void_array_new_instance(self.0) }
    }
    fn allocate(
        &mut self,
        sz: core::ffi::c_longlong,
        ext: core::ffi::c_longlong,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_void_array_allocate(
                sself: *mut core::ffi::c_void,
                sz: core::ffi::c_longlong,
                ext: core::ffi::c_longlong,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_void_array_allocate(self.0, sz, ext) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_void_array_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_void_array_initialize(self.0) }
    }
    fn get_data_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_void_array_get_data_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_void_array_get_data_type(self.0) }
    }
    fn get_data_type_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_void_array_get_data_type_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_void_array_get_data_type_size(self.0) }
    }
    fn set_number_of_pointers(&mut self, number: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_void_array_set_number_of_pointers(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_void_array_set_number_of_pointers(self.0, number) }
    }
    fn get_number_of_pointers(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_void_array_get_number_of_pointers(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_void_array_get_number_of_pointers(self.0) }
    }
    fn reset(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_void_array_reset(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_void_array_reset(self.0) }
    }
    fn squeeze(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_void_array_squeeze(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_void_array_squeeze(self.0) }
    }
    fn deep_copy(&mut self, va: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_void_array_deep_copy(
                sself: *mut core::ffi::c_void,
                va: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_void_array_deep_copy(self.0, va) }
    }
}
impl VtkWeakReference for vtkWeakReference {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_weak_reference_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_weak_reference_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_weak_reference_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_weak_reference_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_weak_reference_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_weak_reference_new(self.0) }
    }
    fn set(&mut self, object: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_weak_reference_set(
                sself: *mut core::ffi::c_void,
                object: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_weak_reference_set(self.0, object) }
    }
    fn get(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_weak_reference_get(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_weak_reference_get(self.0) }
    }
}
impl VtkXMLFileOutputWindow for vtkXMLFileOutputWindow {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_xml_file_output_window_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_xml_file_output_window_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_xml_file_output_window_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_xml_file_output_window_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_xml_file_output_window_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_xml_file_output_window_new(self.0) }
    }
    fn display_text(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_xml_file_output_window_display_text(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_xml_file_output_window_display_text(self.0, c_p0.as_ptr()) }
    }
    fn display_tag(&mut self, p0: &str) -> () {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_xml_file_output_window_display_tag(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_xml_file_output_window_display_tag(self.0, c_p0.as_ptr()) }
    }
}
/// a seqin an animation.
///
///
/// vtkAnimationCue and vtkAnimationScene provide the framework to support
/// animations in VTK. vtkAnimationCue represents an entity that changes/
/// animates with time, while vtkAnimationScene represents scene or setup
/// for the animation, which consists on individual cues or other scenes.
///
/// A cue has three states: UNINITIALIZED, ACTIVE and INACTIVE.
/// UNINITIALIZED represents an point in time before the start time of the cue.
/// The cue is in ACTIVE state at a point in time between start time and end time
/// for the cue. While, beyond the end time, it is in INACTIVE state.
/// When the cue enters the ACTIVE state, StartAnimationCueEvent is fired. This
/// event may be handled to initialize the entity to be animated.
/// When the cue leaves the ACTIVE state, EndAnimationCueEvent is fired, which
/// can be handled to cleanup after having run the animation.
/// For every request to render during the ACTIVE state, AnimationCueTickEvent is
/// fired, which must be handled to perform the actual animation.
/// @sa
/// vtkAnimationScene
#[allow(non_camel_case_types)]
pub struct vtkAnimationCue(*mut core::ffi::c_void);
impl vtkAnimationCue {
    /// Creates a new [vtkAnimationCue] via `vtkAnimationCue::New()`
    #[doc(alias = "vtkAnimationCue")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAnimationCue_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkAnimationCue_new() })
    }
}
impl std::default::Default for vtkAnimationCue {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAnimationCue {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAnimationCue_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAnimationCue_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAnimationCue_create_drop() {
    let obj = vtkAnimationCue::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Writes an archive
///
///
/// vtkArchiver is a base class for constructing an archive. The default
/// implementation constructs a directory at the location of the ArchiveName
/// and populates it with files and directories as requested by Insert().
/// Classes that derive from vtkArchiver can customize the output using such
/// features as compression, in-memory serialization and third-party archival
/// tools.
#[allow(non_camel_case_types)]
pub struct vtkArchiver(*mut core::ffi::c_void);
impl vtkArchiver {
    /// Creates a new [vtkArchiver] via `vtkArchiver::New()`
    #[doc(alias = "vtkArchiver")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkArchiver_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkArchiver_new() })
    }
}
impl std::default::Default for vtkArchiver {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkArchiver {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkArchiver_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkArchiver_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkArchiver_create_drop() {
    let obj = vtkArchiver::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of bits
///
///
/// vtkBitArray is an array of bits (0/1 data value). The array is packed
/// so that each byte stores eight bits. vtkBitArray provides methods
/// for insertion and retrieval of bits, and will automatically resize
/// itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkBitArray(*mut core::ffi::c_void);
impl vtkBitArray {
    /// Creates a new [vtkBitArray] via `vtkBitArray::New()`
    #[doc(alias = "vtkBitArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBitArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkBitArray_new() })
    }
}
impl std::default::Default for vtkBitArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBitArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBitArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBitArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBitArray_create_drop() {
    let obj = vtkBitArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Iterator for vtkBitArray.
///
/// This iterator iterates over a vtkBitArray. It uses the double interface
/// to get/set bit values.
#[allow(non_camel_case_types)]
pub struct vtkBitArrayIterator(*mut core::ffi::c_void);
impl vtkBitArrayIterator {
    /// Creates a new [vtkBitArrayIterator] via `vtkBitArrayIterator::New()`
    #[doc(alias = "vtkBitArrayIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBitArrayIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkBitArrayIterator_new() })
    }
}
impl std::default::Default for vtkBitArrayIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBitArrayIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBitArrayIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBitArrayIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBitArrayIterator_create_drop() {
    let obj = vtkBitArrayIterator::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Gaussian sequence of pseudo random numbers implemented with the Box-Mueller transform
///
///
/// vtkGaussianRandomSequence is a sequence of pseudo random numbers
/// distributed according to the Gaussian/normal distribution (mean=0 and
/// standard deviation=1).
///
/// It based is calculation from a uniformly distributed pseudo random sequence.
/// The initial sequence is a vtkMinimalStandardRandomSequence.
#[allow(non_camel_case_types)]
pub struct vtkBoxMuellerRandomSequence(*mut core::ffi::c_void);
impl vtkBoxMuellerRandomSequence {
    /// Creates a new [vtkBoxMuellerRandomSequence] via `vtkBoxMuellerRandomSequence::New()`
    #[doc(alias = "vtkBoxMuellerRandomSequence")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBoxMuellerRandomSequence_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkBoxMuellerRandomSequence_new() })
    }
}
impl std::default::Default for vtkBoxMuellerRandomSequence {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBoxMuellerRandomSequence {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBoxMuellerRandomSequence_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBoxMuellerRandomSequence_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBoxMuellerRandomSequence_create_drop() {
    let obj = vtkBoxMuellerRandomSequence::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// perform machine dependent byte swapping
///
///
/// vtkByteSwap is used by other classes to perform machine dependent byte
/// swapping. Byte swapping is often used when reading or writing binary
/// files.
#[allow(non_camel_case_types)]
pub struct vtkByteSwap(*mut core::ffi::c_void);
impl vtkByteSwap {
    /// Creates a new [vtkByteSwap] via `vtkByteSwap::New()`
    #[doc(alias = "vtkByteSwap")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkByteSwap_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkByteSwap_new() })
    }
}
impl std::default::Default for vtkByteSwap {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkByteSwap {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkByteSwap_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkByteSwap_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkByteSwap_create_drop() {
    let obj = vtkByteSwap::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// supports function callbacks
///
///
/// Use vtkCallbackCommand for generic function callbacks. That is, this class
/// can be used when you wish to execute a function (of the signature
/// described below) using the Command/Observer design pattern in VTK.
/// The callback function should have the form
/// <pre>
/// void func(vtkObject*, unsigned long eid, void* clientdata, void *calldata)
/// </pre>
/// where the parameter vtkObject* is the object invoking the event; eid is
/// the event id (see vtkCommand.h); clientdata is special data that should
/// is associated with this instance of vtkCallbackCommand; and calldata is
/// data that the vtkObject::InvokeEvent() may send with the callback. For
/// example, the invocation of the ProgressEvent sends along the progress
/// value as calldata.
///
///
/// @sa
/// vtkCommand vtkOldStyleCallbackCommand
#[allow(non_camel_case_types)]
pub struct vtkCallbackCommand(*mut core::ffi::c_void);
impl vtkCallbackCommand {
    /// Creates a new [vtkCallbackCommand] via `vtkCallbackCommand::New()`
    #[doc(alias = "vtkCallbackCommand")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCallbackCommand_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCallbackCommand_new() })
    }
}
impl std::default::Default for vtkCallbackCommand {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCallbackCommand {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCallbackCommand_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCallbackCommand_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCallbackCommand_create_drop() {
    let obj = vtkCallbackCommand::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of char
///
///
/// vtkCharArray is an array of values of type char.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// @warning
/// This class should be avoided in favor of either
/// vtkSignedCharArray or vtkUnsignedCharArray. On some systems
/// the underlying data will be stored as unsigned chars and others
/// it will be stored as signed chars. Additionally, saving this
/// array out and then reading it back in it could be transformed to
/// a vtkSignedCharArray or vtkUnsignedCharArray and if that happens
/// the result of a vtkCharArray::SafeDownCast() of that pointer will be
/// a null pointer.
///
/// @sa
/// vtkSignedCharArray vtkUnsignedCharArray
#[allow(non_camel_case_types)]
pub struct vtkCharArray(*mut core::ffi::c_void);
impl vtkCharArray {
    /// Creates a new [vtkCharArray] via `vtkCharArray::New()`
    #[doc(alias = "vtkCharArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCharArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCharArray_new() })
    }
}
impl std::default::Default for vtkCharArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCharArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCharArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCharArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCharArray_create_drop() {
    let obj = vtkCharArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create and manipulate ordered lists of objects
///
///
/// vtkCollection is a general object for creating and manipulating lists
/// of objects. The lists are ordered and allow duplicate entries.
/// vtkCollection also serves as a base class for lists of specific types
/// of objects.
///
/// @sa
/// vtkActorCollection vtkAssemblyPaths vtkDataSetCollection
/// vtkImplicitFunctionCollection vtkLightCollection vtkPolyDataCollection
/// vtkRenderWindowCollection vtkRendererCollection
/// vtkStructuredPointsCollection vtkTransformCollection vtkVolumeCollection
#[allow(non_camel_case_types)]
pub struct vtkCollection(*mut core::ffi::c_void);
impl vtkCollection {
    /// Creates a new [vtkCollection] via `vtkCollection::New()`
    #[doc(alias = "vtkCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCollection_new() })
    }
}
impl std::default::Default for vtkCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCollection_create_drop() {
    let obj = vtkCollection::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// iterator through a vtkCollection.
///
///
/// vtkCollectionIterator provides an alternative way to traverse
/// through the objects in a vtkCollection.  Unlike the collection's
/// built in interface, this allows multiple iterators to
/// simultaneously traverse the collection.  If items are removed from
/// the collection, only the iterators currently pointing to those
/// items are invalidated.  Other iterators will still continue to
/// function normally.
#[allow(non_camel_case_types)]
pub struct vtkCollectionIterator(*mut core::ffi::c_void);
impl vtkCollectionIterator {
    /// Creates a new [vtkCollectionIterator] via `vtkCollectionIterator::New()`
    #[doc(alias = "vtkCollectionIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCollectionIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCollectionIterator_new() })
    }
}
impl std::default::Default for vtkCollectionIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCollectionIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCollectionIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCollectionIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCollectionIterator_create_drop() {
    let obj = vtkCollectionIterator::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Critical section locking class
///
///
/// vtkCriticalSection allows the locking of variables which are accessed
/// through different threads.  This header file also defines
/// vtkSimpleCriticalSection which is not a subclass of vtkObject.
/// The API is identical to that of vtkMutexLock, and the behavior is
/// identical as well, except on Windows 9x/NT platforms. The only difference
/// on these platforms is that vtkMutexLock is more flexible, in that
/// it works across processes as well as across threads, but also costs
/// more, in that it evokes a 600-cycle x86 ring transition. The
/// vtkCriticalSection provides a higher-performance equivalent (on
/// Windows) but won't work across processes. Since it is unclear how,
/// in vtk, an object at the vtk level can be shared across processes
/// in the first place, one should use vtkCriticalSection unless one has
/// a very good reason to use vtkMutexLock. If higher-performance equivalents
/// for non-Windows platforms (Irix, SunOS, etc) are discovered, they
/// should replace the implementations in this class
#[allow(non_camel_case_types)]
pub struct vtkCriticalSection(*mut core::ffi::c_void);
impl vtkCriticalSection {
    /// Creates a new [vtkCriticalSection] via `vtkCriticalSection::New()`
    #[doc(alias = "vtkCriticalSection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCriticalSection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCriticalSection_new() })
    }
}
impl std::default::Default for vtkCriticalSection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCriticalSection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCriticalSection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCriticalSection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCriticalSection_create_drop() {
    let obj = vtkCriticalSection::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// maintain an ordered list of dataarray objects
///
///
/// vtkDataArrayCollection is an object that creates and manipulates lists of
/// datasets. See also vtkCollection and subclasses.
#[allow(non_camel_case_types)]
pub struct vtkDataArrayCollection(*mut core::ffi::c_void);
impl vtkDataArrayCollection {
    /// Creates a new [vtkDataArrayCollection] via `vtkDataArrayCollection::New()`
    #[doc(alias = "vtkDataArrayCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataArrayCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDataArrayCollection_new() })
    }
}
impl std::default::Default for vtkDataArrayCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataArrayCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataArrayCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataArrayCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataArrayCollection_create_drop() {
    let obj = vtkDataArrayCollection::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// iterator through a vtkDataArrayCollection.
///
///
/// vtkDataArrayCollectionIterator provides an implementation of
/// vtkCollectionIterator which allows the items to be retrieved with
/// the proper subclass pointer type for vtkDataArrayCollection.
#[allow(non_camel_case_types)]
pub struct vtkDataArrayCollectionIterator(*mut core::ffi::c_void);
impl vtkDataArrayCollectionIterator {
    /// Creates a new [vtkDataArrayCollectionIterator] via `vtkDataArrayCollectionIterator::New()`
    #[doc(alias = "vtkDataArrayCollectionIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataArrayCollectionIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDataArrayCollectionIterator_new() })
    }
}
impl std::default::Default for vtkDataArrayCollectionIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataArrayCollectionIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataArrayCollectionIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataArrayCollectionIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataArrayCollectionIterator_create_drop() {
    let obj = vtkDataArrayCollectionIterator::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Store on/off settings for data arrays for a vtkSource.
///
///
/// vtkDataArraySelection can be used by vtkSource subclasses to store
/// on/off settings for whether each vtkDataArray in its input should
/// be passed in the source's output.  This is primarily intended to
/// allow file readers to configure what data arrays are read from the
/// file.
#[allow(non_camel_case_types)]
pub struct vtkDataArraySelection(*mut core::ffi::c_void);
impl vtkDataArraySelection {
    /// Creates a new [vtkDataArraySelection] via `vtkDataArraySelection::New()`
    #[doc(alias = "vtkDataArraySelection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataArraySelection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDataArraySelection_new() })
    }
}
impl std::default::Default for vtkDataArraySelection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataArraySelection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataArraySelection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataArraySelection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataArraySelection_create_drop() {
    let obj = vtkDataArraySelection::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// identify memory leaks at program termination
///
/// vtkDebugLeaks is used to report memory leaks at the exit of the program. It
/// uses vtkObjectBase::InitializeObjectBase() (called via vtkObjectFactory
/// macros) to intercept the construction of all VTK objects. It uses the
/// UnRegisterInternal method of vtkObjectBase to intercept the destruction of
/// all objects.
///
/// If not using the vtkObjectFactory macros to implement New(), be sure to call
/// vtkObjectBase::InitializeObjectBase() explicitly on the constructed
/// instance. The rule of thumb is that wherever "new [some vtkObjectBase
/// subclass]" is called, vtkObjectBase::InitializeObjectBase() must be called
/// as well.
///
/// There are exceptions to this:
///
/// - vtkCommand subclasses traditionally do not fully participate in
/// vtkDebugLeaks registration, likely because they typically do not use
/// vtkTypeMacro to configure GetClassName. InitializeObjectBase should not be
/// called on vtkCommand subclasses, and all such classes will be automatically
/// registered with vtkDebugLeaks as "vtkCommand or subclass".
///
/// - vtkInformationKey subclasses are not reference counted. They are allocated
/// statically and registered automatically with a singleton "manager" instance.
/// The manager ensures that all keys are cleaned up before exiting, and
/// registration/deregistration with vtkDebugLeaks is bypassed.
///
/// A table of object name to number of instances is kept. At the exit of the
/// program if there are still VTK objects around it will print them out. To
/// enable this class add the flag -DVTK_DEBUG_LEAKS to the compile line, and
/// rebuild vtkObject and vtkObjectFactory.
#[allow(non_camel_case_types)]
pub struct vtkDebugLeaks(*mut core::ffi::c_void);
impl vtkDebugLeaks {
    /// Creates a new [vtkDebugLeaks] via `vtkDebugLeaks::New()`
    #[doc(alias = "vtkDebugLeaks")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDebugLeaks_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDebugLeaks_new() })
    }
}
impl std::default::Default for vtkDebugLeaks {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDebugLeaks {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDebugLeaks_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDebugLeaks_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDebugLeaks_create_drop() {
    let obj = vtkDebugLeaks::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of double
///
///
/// vtkDoubleArray is an array of values of type double.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkDoubleArray(*mut core::ffi::c_void);
impl vtkDoubleArray {
    /// Creates a new [vtkDoubleArray] via `vtkDoubleArray::New()`
    #[doc(alias = "vtkDoubleArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDoubleArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDoubleArray_new() })
    }
}
impl std::default::Default for vtkDoubleArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDoubleArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDoubleArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDoubleArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDoubleArray_create_drop() {
    let obj = vtkDoubleArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// class interface to system dynamic libraries
///
///
/// vtkDynamicLoader provides a portable interface to loading dynamic
/// libraries into a process.
/// @sa
/// A more portable and lightweight solution is kwsys::DynamicLoader
#[allow(non_camel_case_types)]
pub struct vtkDynamicLoader(*mut core::ffi::c_void);
impl vtkDynamicLoader {
    /// Creates a new [vtkDynamicLoader] via `vtkDynamicLoader::New()`
    #[doc(alias = "vtkDynamicLoader")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDynamicLoader_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDynamicLoader_new() })
    }
}
impl std::default::Default for vtkDynamicLoader {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDynamicLoader {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDynamicLoader_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDynamicLoader_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDynamicLoader_create_drop() {
    let obj = vtkDynamicLoader::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkEventDataDevice3D(*mut core::ffi::c_void);
impl vtkEventDataDevice3D {
    /// Creates a new [vtkEventDataDevice3D] via `vtkEventDataDevice3D::New()`
    #[doc(alias = "vtkEventDataDevice3D")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEventDataDevice3D_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkEventDataDevice3D_new() })
    }
}
impl std::default::Default for vtkEventDataDevice3D {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEventDataDevice3D {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEventDataDevice3D_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEventDataDevice3D_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEventDataDevice3D_create_drop() {
    let obj = vtkEventDataDevice3D::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkEventDataForDevice(*mut core::ffi::c_void);
impl vtkEventDataForDevice {
    /// Creates a new [vtkEventDataForDevice] via `vtkEventDataForDevice::New()`
    #[doc(alias = "vtkEventDataForDevice")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEventDataForDevice_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkEventDataForDevice_new() })
    }
}
impl std::default::Default for vtkEventDataForDevice {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEventDataForDevice {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEventDataForDevice_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEventDataForDevice_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEventDataForDevice_create_drop() {
    let obj = vtkEventDataForDevice::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// a simple event forwarder command
///
///
/// Use vtkEventForwarderCommand to forward an event to a new object.
/// This command will intercept the event, and use InvokeEvent
/// on a 'target' as if that object was the one that invoked the event instead
/// of the object this command was attached to using AddObserver.
///
/// @sa
/// vtkCommand
#[allow(non_camel_case_types)]
pub struct vtkEventForwarderCommand(*mut core::ffi::c_void);
impl vtkEventForwarderCommand {
    /// Creates a new [vtkEventForwarderCommand] via `vtkEventForwarderCommand::New()`
    #[doc(alias = "vtkEventForwarderCommand")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEventForwarderCommand_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkEventForwarderCommand_new() })
    }
}
impl std::default::Default for vtkEventForwarderCommand {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEventForwarderCommand {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEventForwarderCommand_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEventForwarderCommand_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEventForwarderCommand_create_drop() {
    let obj = vtkEventForwarderCommand::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// File Specific output window class
///
///
/// Writes debug/warning/error output to a log file instead of the console.
/// To use this class, instantiate it and then call SetInstance(this).
#[allow(non_camel_case_types)]
pub struct vtkFileOutputWindow(*mut core::ffi::c_void);
impl vtkFileOutputWindow {
    /// Creates a new [vtkFileOutputWindow] via `vtkFileOutputWindow::New()`
    #[doc(alias = "vtkFileOutputWindow")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFileOutputWindow_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkFileOutputWindow_new() })
    }
}
impl std::default::Default for vtkFileOutputWindow {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFileOutputWindow {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFileOutputWindow_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFileOutputWindow_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFileOutputWindow_create_drop() {
    let obj = vtkFileOutputWindow::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of float
///
///
/// vtkFloatArray is an array of values of type float.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkFloatArray(*mut core::ffi::c_void);
impl vtkFloatArray {
    /// Creates a new [vtkFloatArray] via `vtkFloatArray::New()`
    #[doc(alias = "vtkFloatArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFloatArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkFloatArray_new() })
    }
}
impl std::default::Default for vtkFloatArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFloatArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFloatArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFloatArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFloatArray_create_drop() {
    let obj = vtkFloatArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Detect and break reference loops
///
///
/// vtkGarbageCollector is used by VTK classes that may be involved in
/// reference counting loops (such as Algorithm <-> Executive).  It
/// detects strongly connected components of the reference graph that
/// have been leaked deletes them.  The garbage collector uses the
/// ReportReferences method to search the reference graph and construct
/// a net reference count for each connected component.  If the net
/// reference count is zero the entire set of objects is deleted.
/// Deleting each component may leak other components, which are then
/// collected recursively.
///
/// To enable garbage collection for a class, add these members:
///
/// \code
///
/// public:
/// void Register(vtkObjectBase* o) override
/// {
/// this->RegisterInternal(o, true);
/// }
/// void UnRegister(vtkObjectBase* o) override
/// {
/// this->UnRegisterInternal(o, true);
/// }
///
/// protected:
///
/// void ReportReferences(vtkGarbageCollector* collector) override
/// {
/// // Report references held by this object that may be in a loop.
/// this->Superclass::ReportReferences(collector);
/// vtkGarbageCollectorReport(collector, this->OtherObject, "Other Object");
/// }
/// \endcode
///
/// The implementations should be in the .cxx file in practice.
/// It is important that the reference be reported using the real
/// pointer or smart pointer instance that holds the reference.  When
/// collecting the garbage collector will actually set this pointer to
/// nullptr.  The destructor of the class should be written to deal with
/// this.  It is also expected that an invariant is maintained for any
/// reference that is reported.  The variable holding the reference
/// must always either be nullptr or refer to a fully constructed valid
/// object.  Therefore code like "this->Object->UnRegister(this)" must
/// be avoided if "this->Object" is a reported reference because it
/// is possible that the object is deleted before UnRegister returns
/// but then "this->Object" will be left as a dangling pointer.  Instead
/// use code like
///
/// \code
/// vtkObjectBase* obj = this->Object;
/// this->Object = 0;
/// obj->UnRegister(this);
/// \endcode
///
/// so that the reported reference maintains the invariant.
///
/// If subclassing from a class that already supports garbage
/// collection, one need only provide the ReportReferences method.
#[allow(non_camel_case_types)]
pub struct vtkGarbageCollector(*mut core::ffi::c_void);
impl vtkGarbageCollector {
    /// Creates a new [vtkGarbageCollector] via `vtkGarbageCollector::New()`
    #[doc(alias = "vtkGarbageCollector")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGarbageCollector_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkGarbageCollector_new() })
    }
}
impl std::default::Default for vtkGarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGarbageCollector {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGarbageCollector_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGarbageCollector_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGarbageCollector_create_drop() {
    let obj = vtkGarbageCollector::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// list of point or cell ids
///
///
/// vtkIdList is used to represent and pass data id's between
/// objects. vtkIdList may represent any type of integer id, but
/// usually represents point and cell ids.
#[allow(non_camel_case_types)]
pub struct vtkIdList(*mut core::ffi::c_void);
impl vtkIdList {
    /// Creates a new [vtkIdList] via `vtkIdList::New()`
    #[doc(alias = "vtkIdList")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIdList_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkIdList_new() })
    }
}
impl std::default::Default for vtkIdList {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIdList {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIdList_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIdList_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIdList_create_drop() {
    let obj = vtkIdList::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// maintain an ordered list of IdList objects
///
///
/// vtkIdListCollection is an object that creates and manipulates lists of
/// IdLists. See also vtkCollection and subclasses.
#[allow(non_camel_case_types)]
pub struct vtkIdListCollection(*mut core::ffi::c_void);
impl vtkIdListCollection {
    /// Creates a new [vtkIdListCollection] via `vtkIdListCollection::New()`
    #[doc(alias = "vtkIdListCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIdListCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkIdListCollection_new() })
    }
}
impl std::default::Default for vtkIdListCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIdListCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIdListCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIdListCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIdListCollection_create_drop() {
    let obj = vtkIdListCollection::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of vtkIdType
///
///
/// vtkIdTypeArray is an array of values of type vtkIdType.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkIdTypeArray(*mut core::ffi::c_void);
impl vtkIdTypeArray {
    /// Creates a new [vtkIdTypeArray] via `vtkIdTypeArray::New()`
    #[doc(alias = "vtkIdTypeArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIdTypeArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkIdTypeArray_new() })
    }
}
impl std::default::Default for vtkIdTypeArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIdTypeArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIdTypeArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIdTypeArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIdTypeArray_create_drop() {
    let obj = vtkIdTypeArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Store vtkAlgorithm input/output information.
///
///
/// vtkInformation represents information and/or data for one input or
/// one output of a vtkAlgorithm.  It maps from keys to values of
/// several data types.  Instances of this class are collected in
/// vtkInformationVector instances and passed to
/// vtkAlgorithm::ProcessRequest calls.  The information and
/// data referenced by the instance on a particular input or output
/// define the request made to the vtkAlgorithm instance.
#[allow(non_camel_case_types)]
pub struct vtkInformation(*mut core::ffi::c_void);
impl vtkInformation {
    /// Creates a new [vtkInformation] via `vtkInformation::New()`
    #[doc(alias = "vtkInformation")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInformation_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkInformation_new() })
    }
}
impl std::default::Default for vtkInformation {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInformation {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInformation_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInformation_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInformation_create_drop() {
    let obj = vtkInformation::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Iterates over keys of an information object
///
///
/// vtkInformationIterator can be used to iterate over the keys of an
/// information object. The corresponding values can then be directly
/// obtained from the information object using the keys.
///
/// @sa
/// vtkInformation vtkInformationKey
#[allow(non_camel_case_types)]
pub struct vtkInformationIterator(*mut core::ffi::c_void);
impl vtkInformationIterator {
    /// Creates a new [vtkInformationIterator] via `vtkInformationIterator::New()`
    #[doc(alias = "vtkInformationIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInformationIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkInformationIterator_new() })
    }
}
impl std::default::Default for vtkInformationIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInformationIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInformationIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInformationIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInformationIterator_create_drop() {
    let obj = vtkInformationIterator::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Find vtkInformationKeys from name and
///
/// location strings.
#[allow(non_camel_case_types)]
pub struct vtkInformationKeyLookup(*mut core::ffi::c_void);
impl vtkInformationKeyLookup {
    /// Creates a new [vtkInformationKeyLookup] via `vtkInformationKeyLookup::New()`
    #[doc(alias = "vtkInformationKeyLookup")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInformationKeyLookup_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkInformationKeyLookup_new() })
    }
}
impl std::default::Default for vtkInformationKeyLookup {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInformationKeyLookup {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInformationKeyLookup_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInformationKeyLookup_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInformationKeyLookup_create_drop() {
    let obj = vtkInformationKeyLookup::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Store zero or more vtkInformation instances.
///
///
///
/// vtkInformationVector stores a vector of zero or more vtkInformation
/// objects corresponding to the input or output information for a
/// vtkAlgorithm.  An instance of this class is passed to
/// vtkAlgorithm::ProcessRequest calls.
#[allow(non_camel_case_types)]
pub struct vtkInformationVector(*mut core::ffi::c_void);
impl vtkInformationVector {
    /// Creates a new [vtkInformationVector] via `vtkInformationVector::New()`
    #[doc(alias = "vtkInformationVector")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInformationVector_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkInformationVector_new() })
    }
}
impl std::default::Default for vtkInformationVector {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInformationVector {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInformationVector_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInformationVector_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInformationVector_create_drop() {
    let obj = vtkInformationVector::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of int
///
///
/// vtkIntArray is an array of values of type int.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the int type, so use
/// of this type directly is discouraged.  If an array of 32 bit integers is
/// needed, prefer vtkTypeInt32Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkIntArray(*mut core::ffi::c_void);
impl vtkIntArray {
    /// Creates a new [vtkIntArray] via `vtkIntArray::New()`
    #[doc(alias = "vtkIntArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIntArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkIntArray_new() })
    }
}
impl std::default::Default for vtkIntArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIntArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIntArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIntArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIntArray_create_drop() {
    let obj = vtkIntArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of long
///
///
/// vtkLongArray is an array of values of type long.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the long type, so use
/// of this type directly is discouraged.  If an array of 32 bit integers is
/// needed, prefer vtkTypeInt32Array to this class.  If an array of 64 bit
/// integers is needed, prefer vtkTypeInt64Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkLongArray(*mut core::ffi::c_void);
impl vtkLongArray {
    /// Creates a new [vtkLongArray] via `vtkLongArray::New()`
    #[doc(alias = "vtkLongArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLongArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkLongArray_new() })
    }
}
impl std::default::Default for vtkLongArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLongArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLongArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLongArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLongArray_create_drop() {
    let obj = vtkLongArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of long long
///
///
/// vtkLongLongArray is an array of values of type long long.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// This class should not be used directly, as it only exists on systems
/// where the long long type is defined.  If you need a 64 bit integer
/// data array, use vtkTypeInt64Array instead.
#[allow(non_camel_case_types)]
pub struct vtkLongLongArray(*mut core::ffi::c_void);
impl vtkLongLongArray {
    /// Creates a new [vtkLongLongArray] via `vtkLongLongArray::New()`
    #[doc(alias = "vtkLongLongArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLongLongArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkLongLongArray_new() })
    }
}
impl std::default::Default for vtkLongLongArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLongLongArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLongLongArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLongLongArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLongLongArray_create_drop() {
    let obj = vtkLongLongArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// map scalar values into colors via a lookup table
///
///
/// vtkLookupTable is an object that is used by mapper objects to map scalar
/// values into RGBA (red-green-blue-alpha) color specification,
/// or RGBA into scalar values. The color table can be created by direct
/// insertion of color values, or by specifying a hue, saturation, value, and
/// alpha range and generating a table.
///
/// A special color for NaN values in the data can be specified via
/// SetNanColor(). In addition, a color for data values below the
/// lookup table range minimum can be specified with
/// SetBelowRangeColor(), and that color will be used for values below
/// the range minimum when UseBelowRangeColor is on.  Likewise, a color
/// for data values above the lookup table range maximum can be
/// specified with SetAboveRangeColor(), and it is used when
/// UseAboveRangeColor is on.
///
/// This class behaves differently depending on how \a IndexedLookup is set.
/// When true, vtkLookupTable enters a mode for representing categorical color maps.
/// By setting \a IndexedLookup to true, you indicate that the annotated
/// values are the only valid values for which entries in the color table
/// should be returned. The colors in the lookup \a Table are assigned
/// to annotated values by taking the modulus of their index in the list
/// of annotations. \a IndexedLookup changes the behavior of \a GetIndex,
/// which in turn changes the way \a MapScalarsThroughTable2 behaves;
/// when \a IndexedLookup is true, \a MapScalarsThroughTable2 will search for
/// scalar values in \a AnnotatedValues and use the resulting index to
/// determine the color. If a scalar value is not present in \a AnnotatedValues,
/// then \a NanColor will be used.
///
/// @warning
/// You need to explicitly call Build() when constructing the LUT by hand.
///
/// @sa
/// vtkLogLookupTable vtkWindowLevelLookupTable
#[allow(non_camel_case_types)]
pub struct vtkLookupTable(*mut core::ffi::c_void);
impl vtkLookupTable {
    /// Creates a new [vtkLookupTable] via `vtkLookupTable::New()`
    #[doc(alias = "vtkLookupTable")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLookupTable_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkLookupTable_new() })
    }
}
impl std::default::Default for vtkLookupTable {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLookupTable {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLookupTable_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLookupTable_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLookupTable_create_drop() {
    let obj = vtkLookupTable::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// performs common math operations
///
///
/// vtkMath provides methods to perform common math operations. These
/// include providing constants such as Pi; conversion from degrees to
/// radians; vector operations such as dot and cross products and vector
/// norm; matrix determinant for 2x2 and 3x3 matrices; univariate polynomial
/// solvers; and for random number generation (for backward compatibility only).
/// @sa
/// vtkMinimalStandardRandomSequence, vtkBoxMuellerRandomSequence,
/// vtkQuaternion
#[allow(non_camel_case_types)]
pub struct vtkMath(*mut core::ffi::c_void);
impl vtkMath {
    /// Creates a new [vtkMath] via `vtkMath::New()`
    #[doc(alias = "vtkMath")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMath_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMath_new() })
    }
}
impl std::default::Default for vtkMath {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMath {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMath_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMath_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMath_create_drop() {
    let obj = vtkMath::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generator for Mersenne Twister pseudorandom numbers
///
///
/// vtkMersenneTwister is an implementation of the Mersenne Twister pseudorandom
/// number generator. The VTK class is simply a wrapper around an implementation
/// written by M. Matsumoto, T. Nishimura and M. Saito, whose source code can be
/// found at http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/DC/dc.html.
///
/// This implementation of the Mersenne Twister facilitates the generation and
/// query from multiple independent pseudorandom sequences. Independent sequences
/// are identified by a unique vtkMersenneTwister::SequenceId, which is either
/// generated upon request or passed into the initialization method. This id is
/// factored into the initialization of the Mersenne Twister's initial state, so
/// two sequences with the same seed and different sequence ids will produce
/// different results. Once a sequence is initialized with an associated sequence
/// id, this id is used to obtain values from the sequence.
///
/// This class, besides generating random sequences in sequential order, can
/// also populate a double array of specified size with a random sequence. It
/// will do so using one or more threads depending on the number of values
/// requested to generate.
#[allow(non_camel_case_types)]
pub struct vtkMersenneTwister(*mut core::ffi::c_void);
impl vtkMersenneTwister {
    /// Creates a new [vtkMersenneTwister] via `vtkMersenneTwister::New()`
    #[doc(alias = "vtkMersenneTwister")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMersenneTwister_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMersenneTwister_new() })
    }
}
impl std::default::Default for vtkMersenneTwister {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMersenneTwister {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMersenneTwister_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMersenneTwister_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMersenneTwister_create_drop() {
    let obj = vtkMersenneTwister::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Park and Miller Sequence of pseudo random numbers
///
///
/// vtkMinimalStandardRandomSequence is a sequence of statistically independent
/// pseudo random numbers uniformly distributed between 0.0 and 1.0.
///
/// The sequence is generated by a prime modulus multiplicative linear
/// congruential generator (PMMLCG) or "Lehmer generator" with multiplier 16807
/// and prime modulus 2^(31)-1. The authors calls it
/// "minimal standard random number generator"
///
/// ref: "Random Number Generators: Good Ones are Hard to Find,"
/// by Stephen K. Park and Keith W. Miller in Communications of the ACM,
/// 31, 10 (Oct. 1988) pp. 1192-1201.
/// Code is at page 1195, "Integer version 2"
///
/// Correctness test is described in first column, page 1195:
/// A seed of 1 at step 1 should give a seed of 1043618065 at step 10001.
#[allow(non_camel_case_types)]
pub struct vtkMinimalStandardRandomSequence(*mut core::ffi::c_void);
impl vtkMinimalStandardRandomSequence {
    /// Creates a new [vtkMinimalStandardRandomSequence] via `vtkMinimalStandardRandomSequence::New()`
    #[doc(alias = "vtkMinimalStandardRandomSequence")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMinimalStandardRandomSequence_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMinimalStandardRandomSequence_new() })
    }
}
impl std::default::Default for vtkMinimalStandardRandomSequence {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMinimalStandardRandomSequence {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMinimalStandardRandomSequence_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkMinimalStandardRandomSequence_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMinimalStandardRandomSequence_create_drop() {
    let obj = vtkMinimalStandardRandomSequence::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// A class for performing multithreaded execution
///
///
/// vtkMultithreader is a class that provides support for multithreaded
/// execution using pthreads on POSIX systems, or Win32 threads on
/// Windows.  This class can be used to execute a single
/// method on multiple threads, or to specify a method per thread.
#[allow(non_camel_case_types)]
pub struct vtkMultiThreader(*mut core::ffi::c_void);
impl vtkMultiThreader {
    /// Creates a new [vtkMultiThreader] via `vtkMultiThreader::New()`
    #[doc(alias = "vtkMultiThreader")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMultiThreader_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMultiThreader_new() })
    }
}
impl std::default::Default for vtkMultiThreader {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMultiThreader {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMultiThreader_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMultiThreader_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMultiThreader_create_drop() {
    let obj = vtkMultiThreader::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// abstract base class for most VTK objects
///
///
/// vtkObject is the base class for most objects in the visualization
/// toolkit. vtkObject provides methods for tracking modification time,
/// debugging, printing, and event callbacks. Most objects created
/// within the VTK framework should be a subclass of vtkObject or one
/// of its children.  The few exceptions tend to be very small helper
/// classes that usually never get instantiated or situations where
/// multiple inheritance gets in the way.  vtkObject also performs
/// reference counting: objects that are reference counted exist as
/// long as another object uses them. Once the last reference to a
/// reference counted object is removed, the object will spontaneously
/// destruct.
///
/// @warning
/// Note: in VTK objects should always be created with the New() method
/// and deleted with the Delete() method. VTK objects cannot be
/// allocated off the stack (i.e., automatic objects) because the
/// constructor is a protected method.
///
/// @sa
/// vtkCommand vtkTimeStamp
#[allow(non_camel_case_types)]
pub struct vtkObject(*mut core::ffi::c_void);
impl vtkObject {
    /// Creates a new [vtkObject] via `vtkObject::New()`
    #[doc(alias = "vtkObject")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkObject_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkObject_new() })
    }
}
impl std::default::Default for vtkObject {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkObject {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkObject_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkObject_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkObject_create_drop() {
    let obj = vtkObject::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// maintain a list of object factories
///
///
/// vtkObjectFactoryCollection is an object that creates and manipulates
/// ordered lists of objects of type vtkObjectFactory.
///
/// @sa
/// vtkCollection vtkObjectFactory
#[allow(non_camel_case_types)]
pub struct vtkObjectFactoryCollection(*mut core::ffi::c_void);
impl vtkObjectFactoryCollection {
    /// Creates a new [vtkObjectFactoryCollection] via `vtkObjectFactoryCollection::New()`
    #[doc(alias = "vtkObjectFactoryCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkObjectFactoryCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkObjectFactoryCollection_new() })
    }
}
impl std::default::Default for vtkObjectFactoryCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkObjectFactoryCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkObjectFactoryCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkObjectFactoryCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkObjectFactoryCollection_create_drop() {
    let obj = vtkObjectFactoryCollection::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// supports legacy function callbacks for VTK
///
///
/// vtkOldStyleCallbackCommand is a callback that supports the legacy callback
/// methods found in VTK. For example, the legacy method
/// vtkProcessObject::SetStartMethod() is actually invoked using the
/// command/observer design pattern of VTK, and the vtkOldStyleCallbackCommand
/// is used to provide the legacy functionality. The callback function should
/// have the form void func(void *clientdata), where clientdata is special data
/// that should is associated with this instance of vtkCallbackCommand.
///
/// @warning
/// This is legacy glue. Please do not use; it will be eventually eliminated.
///
/// @sa
/// vtkCommand vtkCallbackCommand
#[allow(non_camel_case_types)]
pub struct vtkOldStyleCallbackCommand(*mut core::ffi::c_void);
impl vtkOldStyleCallbackCommand {
    /// Creates a new [vtkOldStyleCallbackCommand] via `vtkOldStyleCallbackCommand::New()`
    #[doc(alias = "vtkOldStyleCallbackCommand")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOldStyleCallbackCommand_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkOldStyleCallbackCommand_new() })
    }
}
impl std::default::Default for vtkOldStyleCallbackCommand {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOldStyleCallbackCommand {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOldStyleCallbackCommand_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOldStyleCallbackCommand_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOldStyleCallbackCommand_create_drop() {
    let obj = vtkOldStyleCallbackCommand::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// base class for writing debug output to a console
///
///
/// This class is used to encapsulate all text output, so that it will work
/// with operating systems that have a stdout and stderr, and ones that
/// do not.  (i.e windows does not).  Sub-classes can be provided which can
/// redirect the output to a window.
#[allow(non_camel_case_types)]
pub struct vtkOutputWindow(*mut core::ffi::c_void);
impl vtkOutputWindow {
    /// Creates a new [vtkOutputWindow] via `vtkOutputWindow::New()`
    #[doc(alias = "vtkOutputWindow")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOutputWindow_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkOutputWindow_new() })
    }
}
impl std::default::Default for vtkOutputWindow {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOutputWindow {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOutputWindow_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOutputWindow_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOutputWindow_create_drop() {
    let obj = vtkOutputWindow::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// maintain a list of override information objects
///
///
/// vtkOverrideInformationCollection is an object that creates and manipulates
/// lists of objects of type vtkOverrideInformation.
/// @sa
/// vtkCollection
#[allow(non_camel_case_types)]
pub struct vtkOverrideInformationCollection(*mut core::ffi::c_void);
impl vtkOverrideInformationCollection {
    /// Creates a new [vtkOverrideInformationCollection] via `vtkOverrideInformationCollection::New()`
    #[doc(alias = "vtkOverrideInformationCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOverrideInformationCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkOverrideInformationCollection_new() })
    }
}
impl std::default::Default for vtkOverrideInformationCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOverrideInformationCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOverrideInformationCollection_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkOverrideInformationCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOverrideInformationCollection_create_drop() {
    let obj = vtkOverrideInformationCollection::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// represent and manipulate 3D points
///
///
/// vtkPoints represents 3D points. The data model for vtkPoints is an
/// array of vx-vy-vz triplets accessible by (point or cell) id.
#[allow(non_camel_case_types)]
pub struct vtkPoints(*mut core::ffi::c_void);
impl vtkPoints {
    /// Creates a new [vtkPoints] via `vtkPoints::New()`
    #[doc(alias = "vtkPoints")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPoints_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPoints_new() })
    }
}
impl std::default::Default for vtkPoints {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPoints {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPoints_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPoints_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPoints_create_drop() {
    let obj = vtkPoints::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// represent and manipulate 2D points
///
///
/// vtkPoints2D represents 2D points. The data model for vtkPoints2D is an
/// array of vx-vy doublets accessible by (point or cell) id.
#[allow(non_camel_case_types)]
pub struct vtkPoints2D(*mut core::ffi::c_void);
impl vtkPoints2D {
    /// Creates a new [vtkPoints2D] via `vtkPoints2D::New()`
    #[doc(alias = "vtkPoints2D")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPoints2D_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPoints2D_new() })
    }
}
impl std::default::Default for vtkPoints2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPoints2D {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPoints2D_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPoints2D_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPoints2D_create_drop() {
    let obj = vtkPoints2D::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// a list of ids arranged in priority order
///
///
/// vtkPriorityQueue is a general object for creating and manipulating lists
/// of object ids (e.g., point or cell ids). Object ids are sorted according
/// to a user-specified priority, where entries at the top of the queue have
/// the smallest values.
///
/// This implementation provides a feature beyond the usual ability to insert
/// and retrieve (or pop) values from the queue. It is also possible to
/// pop any item in the queue given its id number. This allows you to delete
/// entries in the queue which can useful for reinserting an item into the
/// queue.
///
/// @warning
/// This implementation is a variation of the priority queue described in
/// "Data Structures & Algorithms" by Aho, Hopcroft, Ullman. It creates
/// a balanced, partially ordered binary tree implemented as an ordered
/// array. This avoids the overhead associated with parent/child pointers,
/// and frequent memory allocation and deallocation.
#[allow(non_camel_case_types)]
pub struct vtkPriorityQueue(*mut core::ffi::c_void);
impl vtkPriorityQueue {
    /// Creates a new [vtkPriorityQueue] via `vtkPriorityQueue::New()`
    #[doc(alias = "vtkPriorityQueue")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPriorityQueue_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPriorityQueue_new() })
    }
}
impl std::default::Default for vtkPriorityQueue {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPriorityQueue {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPriorityQueue_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPriorityQueue_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPriorityQueue_create_drop() {
    let obj = vtkPriorityQueue::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// convenience class to quickly generate a pool of random numbers
///
///
/// vtkRandomPool generates random numbers, and can do so using
/// multithreading.  It supports parallel applications where generating random
/// numbers on the fly is difficult (i.e., non-deterministic). Also, it can be
/// used to populate vtkDataArrays in an efficient manner. By default it uses
/// an instance of vtkMersenneTwister to generate random sequences, but any
/// subclass of vtkRandomSequence may be used. It also supports simple methods
/// to generate, access, and pass random memory pools between objects.
///
/// In threaded applications, these class may be conveniently used to
/// pre-generate a sequence of random numbers, followed by the use of
/// deterministic accessor methods to produce random sequences without
/// problems etc. due to unpredictable work load and order of thread
/// execution.
///
/// @warning
/// The class uses vtkMultiThreader if the size of the pool is larger than
/// the specified chunk size. Also, vtkSMPTools may be used to scale the
/// components in the method PopulateDataArray().
#[allow(non_camel_case_types)]
pub struct vtkRandomPool(*mut core::ffi::c_void);
impl vtkRandomPool {
    /// Creates a new [vtkRandomPool] via `vtkRandomPool::New()`
    #[doc(alias = "vtkRandomPool")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRandomPool_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkRandomPool_new() })
    }
}
impl std::default::Default for vtkRandomPool {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRandomPool {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRandomPool_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRandomPool_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRandomPool_create_drop() {
    let obj = vtkRandomPool::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Obsolete / empty subclass of object.
///
///
/// vtkReferenceCount functionality has now been moved into vtkObject
/// @sa
/// vtkObject
#[allow(non_camel_case_types)]
pub struct vtkReferenceCount(*mut core::ffi::c_void);
impl vtkReferenceCount {
    /// Creates a new [vtkReferenceCount] via `vtkReferenceCount::New()`
    #[doc(alias = "vtkReferenceCount")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkReferenceCount_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkReferenceCount_new() })
    }
}
impl std::default::Default for vtkReferenceCount {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkReferenceCount {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkReferenceCount_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkReferenceCount_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkReferenceCount_create_drop() {
    let obj = vtkReferenceCount::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for mapping scalar values to colors
///
///
/// vtkScalarsToColors is a general-purpose base class for objects that
/// convert scalars to colors. This include vtkLookupTable classes and
/// color transfer functions.  By itself, this class will simply rescale
/// the scalars.
///
/// The scalar-to-color mapping can be augmented with an additional
/// uniform alpha blend. This is used, for example, to blend a vtkActor's
/// opacity with the lookup table values.
///
/// Specific scalar values may be annotated with text strings that will
/// be included in color legends using \a SetAnnotations, \a SetAnnotation,
/// \a GetNumberOfAnnotatedValues, \a GetAnnotatedValue, \a GetAnnotation,
/// \a RemoveAnnotation, and \a ResetAnnotations.
///
/// This class also has a method for indicating that the set of
/// annotated values form a categorical color map; by setting \a
/// IndexedLookup to true, you indicate that the annotated values are
/// the only valid values for which entries in the color table should
/// be returned. In this mode, subclasses should then assign colors to
/// annotated values by taking the modulus of an annotated value's
/// index in the list of annotations with the number of colors in the
/// table.
///
/// @sa
/// vtkLookupTable vtkColorTransferFunction
#[allow(non_camel_case_types)]
pub struct vtkScalarsToColors(*mut core::ffi::c_void);
impl vtkScalarsToColors {
    /// Creates a new [vtkScalarsToColors] via `vtkScalarsToColors::New()`
    #[doc(alias = "vtkScalarsToColors")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkScalarsToColors_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkScalarsToColors_new() })
    }
}
impl std::default::Default for vtkScalarsToColors {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkScalarsToColors {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkScalarsToColors_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkScalarsToColors_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkScalarsToColors_create_drop() {
    let obj = vtkScalarsToColors::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of short
///
///
/// vtkShortArray is an array of values of type short.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the short type,
/// so use of this type directly is discouraged.  If an array of 16 bit
/// integers is needed, prefer vtkTypeInt16Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkShortArray(*mut core::ffi::c_void);
impl vtkShortArray {
    /// Creates a new [vtkShortArray] via `vtkShortArray::New()`
    #[doc(alias = "vtkShortArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkShortArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkShortArray_new() })
    }
}
impl std::default::Default for vtkShortArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkShortArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkShortArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkShortArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkShortArray_create_drop() {
    let obj = vtkShortArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of signed char
///
///
/// vtkSignedCharArray is an array of values of type signed char.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkSignedCharArray(*mut core::ffi::c_void);
impl vtkSignedCharArray {
    /// Creates a new [vtkSignedCharArray] via `vtkSignedCharArray::New()`
    #[doc(alias = "vtkSignedCharArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSignedCharArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSignedCharArray_new() })
    }
}
impl std::default::Default for vtkSignedCharArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSignedCharArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSignedCharArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSignedCharArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSignedCharArray_create_drop() {
    let obj = vtkSignedCharArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// provides several methods for sorting VTK arrays.
///
///
///
/// vtkSortDataArray is used to sort data, based on its value, or with an
/// associated key, into either ascending or descending order. This is useful
/// for operations like selection, or analysis, when evaluating and processing
/// data. A variety of sorting functions are provided, treating both arrays
/// (i.e., vtkAbstractArray) and id lists (vtkIdList). Note that complex arrays
/// like variants and string arrays are also handled.
///
/// Additional functionality is provided to generate data ordering, without
/// necessarily shuffling the data into a final, sorted position. Hence, the
/// sorting process is organized into three steps because of the complexity of
/// dealing with multiple types and multiple component data arrays. The first
/// step involves creating and initializing a sorted index array, and then
/// (second step) sorting this array to produce a map indicating the sorting
/// order.  In other words, the sorting index array is a permutation which can
/// be applied to other, associated data to shuffle it (third step) into an
/// order consistent with the sorting operation. Note that the generation of
/// the sorted index array is useful unto itself (even without the final
/// shuffling of data) because it generates an ordered list (from the data
/// values of any component in any array). So for example, it is possible to
/// find the top N cells with the largest scalar value simply by generating
/// the sorting index array from the call scalar values.
///
/// @warning
/// This class has been threaded with vtkSMPTools. Using TBB or other
/// non-sequential type (set in the CMake variable
/// VTK_SMP_IMPLEMENTATION_TYPE) may improve performance significantly on
/// multi-core machines.
///
/// @warning
/// The sort methods below are static, hence the sorting methods can be
/// used without instantiating the class. All methods are thread safe.
///
/// @sa
/// vtkSortFieldData
#[allow(non_camel_case_types)]
pub struct vtkSortDataArray(*mut core::ffi::c_void);
impl vtkSortDataArray {
    /// Creates a new [vtkSortDataArray] via `vtkSortDataArray::New()`
    #[doc(alias = "vtkSortDataArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSortDataArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSortDataArray_new() })
    }
}
impl std::default::Default for vtkSortDataArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSortDataArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSortDataArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSortDataArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSortDataArray_create_drop() {
    let obj = vtkSortDataArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// a vtkAbstractArray subclass for strings
///
///
/// Points and cells may sometimes have associated data that are stored
/// as strings, e.g. labels for information visualization projects.
/// This class provides a clean way to store and access those strings.
/// @par Thanks:
/// Andy Wilson (atwilso@sandia.gov) wrote this class.
#[allow(non_camel_case_types)]
pub struct vtkStringArray(*mut core::ffi::c_void);
impl vtkStringArray {
    /// Creates a new [vtkStringArray] via `vtkStringArray::New()`
    #[doc(alias = "vtkStringArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStringArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkStringArray_new() })
    }
}
impl std::default::Default for vtkStringArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStringArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStringArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStringArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStringArray_create_drop() {
    let obj = vtkStringArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// File Specific output window class
///
///
/// Writes debug/warning/error output to a log file instead of the console.
/// To use this class, instantiate it and then call SetInstance(this).
#[allow(non_camel_case_types)]
pub struct vtkStringOutputWindow(*mut core::ffi::c_void);
impl vtkStringOutputWindow {
    /// Creates a new [vtkStringOutputWindow] via `vtkStringOutputWindow::New()`
    #[doc(alias = "vtkStringOutputWindow")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStringOutputWindow_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkStringOutputWindow_new() })
    }
}
impl std::default::Default for vtkStringOutputWindow {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStringOutputWindow {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStringOutputWindow_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStringOutputWindow_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStringOutputWindow_create_drop() {
    let obj = vtkStringOutputWindow::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// performs common time operations
///
///
///
/// vtkTimePointUtility is provides methods to perform common time operations.
#[allow(non_camel_case_types)]
pub struct vtkTimePointUtility(*mut core::ffi::c_void);
impl vtkTimePointUtility {
    /// Creates a new [vtkTimePointUtility] via `vtkTimePointUtility::New()`
    #[doc(alias = "vtkTimePointUtility")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTimePointUtility_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTimePointUtility_new() })
    }
}
impl std::default::Default for vtkTimePointUtility {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTimePointUtility {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTimePointUtility_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTimePointUtility_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTimePointUtility_create_drop() {
    let obj = vtkTimePointUtility::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeFloat32Array(*mut core::ffi::c_void);
impl vtkTypeFloat32Array {
    /// Creates a new [vtkTypeFloat32Array] via `vtkTypeFloat32Array::New()`
    #[doc(alias = "vtkTypeFloat32Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeFloat32Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeFloat32Array_new() })
    }
}
impl std::default::Default for vtkTypeFloat32Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeFloat32Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeFloat32Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeFloat32Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeFloat32Array_create_drop() {
    let obj = vtkTypeFloat32Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeFloat64Array(*mut core::ffi::c_void);
impl vtkTypeFloat64Array {
    /// Creates a new [vtkTypeFloat64Array] via `vtkTypeFloat64Array::New()`
    #[doc(alias = "vtkTypeFloat64Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeFloat64Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeFloat64Array_new() })
    }
}
impl std::default::Default for vtkTypeFloat64Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeFloat64Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeFloat64Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeFloat64Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeFloat64Array_create_drop() {
    let obj = vtkTypeFloat64Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeInt16Array(*mut core::ffi::c_void);
impl vtkTypeInt16Array {
    /// Creates a new [vtkTypeInt16Array] via `vtkTypeInt16Array::New()`
    #[doc(alias = "vtkTypeInt16Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeInt16Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeInt16Array_new() })
    }
}
impl std::default::Default for vtkTypeInt16Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeInt16Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeInt16Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeInt16Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeInt16Array_create_drop() {
    let obj = vtkTypeInt16Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeInt32Array(*mut core::ffi::c_void);
impl vtkTypeInt32Array {
    /// Creates a new [vtkTypeInt32Array] via `vtkTypeInt32Array::New()`
    #[doc(alias = "vtkTypeInt32Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeInt32Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeInt32Array_new() })
    }
}
impl std::default::Default for vtkTypeInt32Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeInt32Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeInt32Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeInt32Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeInt32Array_create_drop() {
    let obj = vtkTypeInt32Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeInt64Array(*mut core::ffi::c_void);
impl vtkTypeInt64Array {
    /// Creates a new [vtkTypeInt64Array] via `vtkTypeInt64Array::New()`
    #[doc(alias = "vtkTypeInt64Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeInt64Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeInt64Array_new() })
    }
}
impl std::default::Default for vtkTypeInt64Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeInt64Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeInt64Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeInt64Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeInt64Array_create_drop() {
    let obj = vtkTypeInt64Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeInt8Array(*mut core::ffi::c_void);
impl vtkTypeInt8Array {
    /// Creates a new [vtkTypeInt8Array] via `vtkTypeInt8Array::New()`
    #[doc(alias = "vtkTypeInt8Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeInt8Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeInt8Array_new() })
    }
}
impl std::default::Default for vtkTypeInt8Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeInt8Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeInt8Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeInt8Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeInt8Array_create_drop() {
    let obj = vtkTypeInt8Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeUInt16Array(*mut core::ffi::c_void);
impl vtkTypeUInt16Array {
    /// Creates a new [vtkTypeUInt16Array] via `vtkTypeUInt16Array::New()`
    #[doc(alias = "vtkTypeUInt16Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeUInt16Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeUInt16Array_new() })
    }
}
impl std::default::Default for vtkTypeUInt16Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeUInt16Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeUInt16Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeUInt16Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeUInt16Array_create_drop() {
    let obj = vtkTypeUInt16Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeUInt32Array(*mut core::ffi::c_void);
impl vtkTypeUInt32Array {
    /// Creates a new [vtkTypeUInt32Array] via `vtkTypeUInt32Array::New()`
    #[doc(alias = "vtkTypeUInt32Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeUInt32Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeUInt32Array_new() })
    }
}
impl std::default::Default for vtkTypeUInt32Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeUInt32Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeUInt32Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeUInt32Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeUInt32Array_create_drop() {
    let obj = vtkTypeUInt32Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeUInt64Array(*mut core::ffi::c_void);
impl vtkTypeUInt64Array {
    /// Creates a new [vtkTypeUInt64Array] via `vtkTypeUInt64Array::New()`
    #[doc(alias = "vtkTypeUInt64Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeUInt64Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeUInt64Array_new() })
    }
}
impl std::default::Default for vtkTypeUInt64Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeUInt64Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeUInt64Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeUInt64Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeUInt64Array_create_drop() {
    let obj = vtkTypeUInt64Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeUInt8Array(*mut core::ffi::c_void);
impl vtkTypeUInt8Array {
    /// Creates a new [vtkTypeUInt8Array] via `vtkTypeUInt8Array::New()`
    #[doc(alias = "vtkTypeUInt8Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeUInt8Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTypeUInt8Array_new() })
    }
}
impl std::default::Default for vtkTypeUInt8Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeUInt8Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeUInt8Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeUInt8Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeUInt8Array_create_drop() {
    let obj = vtkTypeUInt8Array::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Subclass of vtkAbstractArray that holds vtkUnicodeStrings
///
///
///
///
/// @par Thanks:
/// Developed by Timothy M. Shead (tshead@sandia.gov) at Sandia National Laboratories.
#[allow(non_camel_case_types)]
pub struct vtkUnicodeStringArray(*mut core::ffi::c_void);
impl vtkUnicodeStringArray {
    /// Creates a new [vtkUnicodeStringArray] via `vtkUnicodeStringArray::New()`
    #[doc(alias = "vtkUnicodeStringArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnicodeStringArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUnicodeStringArray_new() })
    }
}
impl std::default::Default for vtkUnicodeStringArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnicodeStringArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnicodeStringArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnicodeStringArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnicodeStringArray_create_drop() {
    let obj = vtkUnicodeStringArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of unsigned char
///
///
/// vtkUnsignedCharArray is an array of values of type unsigned char.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedCharArray(*mut core::ffi::c_void);
impl vtkUnsignedCharArray {
    /// Creates a new [vtkUnsignedCharArray] via `vtkUnsignedCharArray::New()`
    #[doc(alias = "vtkUnsignedCharArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedCharArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUnsignedCharArray_new() })
    }
}
impl std::default::Default for vtkUnsignedCharArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedCharArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedCharArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedCharArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedCharArray_create_drop() {
    let obj = vtkUnsignedCharArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of unsigned int
///
///
/// vtkUnsignedIntArray is an array of values of type unsigned int.  It
/// provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the unsigned int type,
/// so use of this type directly is discouraged.  If an array of 32 bit unsigned
/// integers is needed, prefer vtkTypeUInt32Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedIntArray(*mut core::ffi::c_void);
impl vtkUnsignedIntArray {
    /// Creates a new [vtkUnsignedIntArray] via `vtkUnsignedIntArray::New()`
    #[doc(alias = "vtkUnsignedIntArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedIntArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUnsignedIntArray_new() })
    }
}
impl std::default::Default for vtkUnsignedIntArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedIntArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedIntArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedIntArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedIntArray_create_drop() {
    let obj = vtkUnsignedIntArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of unsigned long
///
///
/// vtkUnsignedLongArray is an array of values of type unsigned long.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the unsigned long type,
/// so use of this type directly is discouraged.  If an array of 32 bit
/// unsigned integers is needed, prefer vtkTypeUInt32Array to this class.
/// If an array of 64 bit unsigned integers is needed, prefer
/// vtkUTypeInt64Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedLongArray(*mut core::ffi::c_void);
impl vtkUnsignedLongArray {
    /// Creates a new [vtkUnsignedLongArray] via `vtkUnsignedLongArray::New()`
    #[doc(alias = "vtkUnsignedLongArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedLongArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUnsignedLongArray_new() })
    }
}
impl std::default::Default for vtkUnsignedLongArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedLongArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedLongArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedLongArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedLongArray_create_drop() {
    let obj = vtkUnsignedLongArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of unsigned long long
///
///
/// vtkUnsignedLongLongArray is an array of values of type unsigned long long.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// This class should not be used directly, as it only exists on systems
/// where the unsigned long long type is defined.  If you need an unsigned
/// 64 bit integer data array, use vtkTypeUInt64Array instead.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedLongLongArray(*mut core::ffi::c_void);
impl vtkUnsignedLongLongArray {
    /// Creates a new [vtkUnsignedLongLongArray] via `vtkUnsignedLongLongArray::New()`
    #[doc(alias = "vtkUnsignedLongLongArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedLongLongArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUnsignedLongLongArray_new() })
    }
}
impl std::default::Default for vtkUnsignedLongLongArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedLongLongArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedLongLongArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedLongLongArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedLongLongArray_create_drop() {
    let obj = vtkUnsignedLongLongArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of unsigned short
///
///
/// vtkUnsignedShortArray is an array of values of type unsigned short.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the unsigned short type,
/// so use of this type directly is discouraged.  If an array of 16 bit
/// unsigned integers is needed, prefer vtkTypeUInt16Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedShortArray(*mut core::ffi::c_void);
impl vtkUnsignedShortArray {
    /// Creates a new [vtkUnsignedShortArray] via `vtkUnsignedShortArray::New()`
    #[doc(alias = "vtkUnsignedShortArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedShortArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUnsignedShortArray_new() })
    }
}
impl std::default::Default for vtkUnsignedShortArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedShortArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedShortArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedShortArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedShortArray_create_drop() {
    let obj = vtkUnsignedShortArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// An array holding vtkVariants.
///
///
///
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkVariantArray(*mut core::ffi::c_void);
impl vtkVariantArray {
    /// Creates a new [vtkVariantArray] via `vtkVariantArray::New()`
    #[doc(alias = "vtkVariantArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVariantArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkVariantArray_new() })
    }
}
impl std::default::Default for vtkVariantArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVariantArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVariantArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVariantArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVariantArray_create_drop() {
    let obj = vtkVariantArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Versioning class for vtk
///
///
/// Holds methods for defining/determining the current vtk version
/// (major, minor, build).
///
/// @warning
/// This file will change frequently to update the VTKSourceVersion which
/// timestamps a particular source release.
#[allow(non_camel_case_types)]
pub struct vtkVersion(*mut core::ffi::c_void);
impl vtkVersion {
    /// Creates a new [vtkVersion] via `vtkVersion::New()`
    #[doc(alias = "vtkVersion")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVersion_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkVersion_new() })
    }
}
impl std::default::Default for vtkVersion {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVersion {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVersion_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVersion_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVersion_create_drop() {
    let obj = vtkVersion::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// dynamic, self-adjusting array of void* pointers
///
///
/// vtkVoidArray is an array of pointers to void. It provides methods
/// for insertion and retrieval of these pointers values, and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkVoidArray(*mut core::ffi::c_void);
impl vtkVoidArray {
    /// Creates a new [vtkVoidArray] via `vtkVoidArray::New()`
    #[doc(alias = "vtkVoidArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVoidArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkVoidArray_new() })
    }
}
impl std::default::Default for vtkVoidArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVoidArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVoidArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVoidArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVoidArray_create_drop() {
    let obj = vtkVoidArray::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Utility class to hold a weak reference to a vtkObject.
///
///
/// Simple Set(...)/Get(...) interface. Used in numpy support to provide a
/// reference to a vtkObject without preventing it from being collected.
#[allow(non_camel_case_types)]
pub struct vtkWeakReference(*mut core::ffi::c_void);
impl vtkWeakReference {
    /// Creates a new [vtkWeakReference] via `vtkWeakReference::New()`
    #[doc(alias = "vtkWeakReference")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkWeakReference_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkWeakReference_new() })
    }
}
impl std::default::Default for vtkWeakReference {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkWeakReference {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkWeakReference_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkWeakReference_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkWeakReference_create_drop() {
    let obj = vtkWeakReference::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// XML File Specific output window class
///
///
/// Writes debug/warning/error output to an XML file. Uses prefined XML
/// tags for each text display method. The text is processed to replace
/// XML markup characters.
///
/// DisplayText - \<Text\>
///
/// DisplayErrorText - \<Error\>
///
/// DisplayWarningText - \<Warning\>
///
/// DisplayGenericWarningText - \<GenericWarning\>
///
/// DisplayDebugText - \<Debug\>
///
/// The method DisplayTag outputs the text unprocessed. To use this
/// class, instantiate it and then call SetInstance(this).
#[allow(non_camel_case_types)]
pub struct vtkXMLFileOutputWindow(*mut core::ffi::c_void);
impl vtkXMLFileOutputWindow {
    /// Creates a new [vtkXMLFileOutputWindow] via `vtkXMLFileOutputWindow::New()`
    #[doc(alias = "vtkXMLFileOutputWindow")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkXMLFileOutputWindow_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkXMLFileOutputWindow_new() })
    }
}
impl std::default::Default for vtkXMLFileOutputWindow {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkXMLFileOutputWindow {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkXMLFileOutputWindow_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkXMLFileOutputWindow_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkXMLFileOutputWindow_create_drop() {
    let obj = vtkXMLFileOutputWindow::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
