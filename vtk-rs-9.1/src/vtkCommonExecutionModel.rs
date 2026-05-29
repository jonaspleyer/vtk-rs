pub trait VtkAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn has_executive(&mut self) -> core::ffi::c_int;
    fn get_executive(&mut self) -> *mut core::ffi::c_void;
    fn set_executive(&mut self, executive: *mut core::ffi::c_void) -> ();
    fn process_request(
        &mut self,
        request: *mut core::ffi::c_void,
        inInfo: *mut core::ffi::c_void,
        outInfo: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn modify_request(
        &mut self,
        request: *mut core::ffi::c_void,
        when: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_input_port_information(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_output_port_information(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_information(&mut self) -> *mut core::ffi::c_void;
    fn set_information(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_number_of_input_ports(&mut self) -> core::ffi::c_int;
    fn get_number_of_output_ports(&mut self) -> core::ffi::c_int;
    fn register(&mut self, o: *mut core::ffi::c_void) -> ();
    fn set_abort_execute(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_abort_execute(&mut self) -> core::ffi::c_int;
    fn abort_execute_on(&mut self) -> ();
    fn abort_execute_off(&mut self) -> ();
    fn get_progress(&mut self) -> core::ffi::c_double;
    fn set_progress(&mut self, p0: core::ffi::c_double) -> ();
    fn update_progress(&mut self, amount: core::ffi::c_double) -> ();
    fn set_progress_shift_scale(
        &mut self,
        shift: core::ffi::c_double,
        scale: core::ffi::c_double,
    ) -> ();
    fn get_progress_shift(&mut self) -> core::ffi::c_double;
    fn get_progress_scale(&mut self) -> core::ffi::c_double;
    fn set_progress_text(&mut self, ptext: &str) -> ();
    fn get_error_code(&mut self) -> core::ffi::c_ulong;
    fn input_is_optional(&mut self) -> *mut core::ffi::c_void;
    fn input_is_repeatable(&mut self) -> *mut core::ffi::c_void;
    fn input_required_fields(&mut self) -> *mut core::ffi::c_void;
    fn input_required_data_type(&mut self) -> *mut core::ffi::c_void;
    fn input_arrays_to_process(&mut self) -> *mut core::ffi::c_void;
    fn input_port(&mut self) -> *mut core::ffi::c_void;
    fn input_connection(&mut self) -> *mut core::ffi::c_void;
    fn can_produce_sub_extent(&mut self) -> *mut core::ffi::c_void;
    fn can_handle_piece_request(&mut self) -> *mut core::ffi::c_void;
    fn set_input_array_to_process(
        &mut self,
        idx: core::ffi::c_int,
        port: core::ffi::c_int,
        connection: core::ffi::c_int,
        fieldAssociation: core::ffi::c_int,
        name: &str,
    ) -> ();
    fn get_input_array_information(
        &mut self,
        idx: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn remove_all_inputs(&mut self) -> ();
    fn get_output_data_object(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_input_data_object(
        &mut self,
        port: core::ffi::c_int,
        connection: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_input_connection(
        &mut self,
        port: core::ffi::c_int,
        input: *mut core::ffi::c_void,
    ) -> ();
    fn add_input_connection(
        &mut self,
        port: core::ffi::c_int,
        input: *mut core::ffi::c_void,
    ) -> ();
    fn remove_input_connection(
        &mut self,
        port: core::ffi::c_int,
        input: *mut core::ffi::c_void,
    ) -> ();
    fn remove_all_input_connections(&mut self, port: core::ffi::c_int) -> ();
    fn set_input_data_object(
        &mut self,
        port: core::ffi::c_int,
        data: *mut core::ffi::c_void,
    ) -> ();
    fn add_input_data_object(
        &mut self,
        port: core::ffi::c_int,
        data: *mut core::ffi::c_void,
    ) -> ();
    fn get_output_port(&mut self, index: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn get_number_of_input_connections(
        &mut self,
        port: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_total_number_of_input_connections(&mut self) -> core::ffi::c_int;
    fn get_input_connection(
        &mut self,
        port: core::ffi::c_int,
        index: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_input_algorithm(
        &mut self,
        port: core::ffi::c_int,
        index: core::ffi::c_int,
        algPort: &mut core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_input_executive(
        &mut self,
        port: core::ffi::c_int,
        index: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_input_information(
        &mut self,
        port: core::ffi::c_int,
        index: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_output_information(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn update(&mut self, port: core::ffi::c_int) -> ();
    fn update_information(&mut self) -> ();
    fn update_data_object(&mut self) -> ();
    fn propagate_update_extent(&mut self) -> ();
    fn update_whole_extent(&mut self) -> ();
    fn convert_total_input_to_port_connection(
        &mut self,
        ind: core::ffi::c_int,
        port: &mut core::ffi::c_int,
        conn: &mut core::ffi::c_int,
    ) -> ();
    fn set_release_data_flag(&mut self, p0: core::ffi::c_int) -> ();
    fn get_release_data_flag(&mut self) -> core::ffi::c_int;
    fn release_data_flag_on(&mut self) -> ();
    fn release_data_flag_off(&mut self) -> ();
    fn update_extent_is_empty(
        &mut self,
        pinfo: *mut core::ffi::c_void,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn set_default_executive_prototype(&mut self, proto: *mut core::ffi::c_void) -> ();
    fn get_update_piece(&mut self) -> core::ffi::c_int;
    fn get_update_number_of_pieces(&mut self) -> core::ffi::c_int;
    fn get_update_ghost_level(&mut self) -> core::ffi::c_int;
    fn set_progress_observer(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_progress_observer(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkAlgorithmOutput {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_index(&mut self, index: core::ffi::c_int) -> ();
    fn get_index(&mut self) -> core::ffi::c_int;
    fn get_producer(&mut self) -> *mut core::ffi::c_void;
    fn set_producer(&mut self, producer: *mut core::ffi::c_void) -> ();
}
pub trait VtkAnnotationLayersAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkArrayDataAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkCachedStreamingDemandDrivenPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_cache_size(&mut self, size: core::ffi::c_int) -> ();
    fn get_cache_size(&mut self) -> core::ffi::c_int;
}
pub trait VtkCastToConcrete {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkCompositeDataPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_composite_output_data(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn load_requested_blocks(&mut self) -> *mut core::ffi::c_void;
    fn composite_data_meta_data(&mut self) -> *mut core::ffi::c_void;
    fn update_composite_indices(&mut self) -> *mut core::ffi::c_void;
    fn block_amount_of_detail(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkCompositeDataSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkDataObjectAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkDataSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_points_output(&mut self) -> *mut core::ffi::c_void;
    fn get_image_data_output(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_rectilinear_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkDemandDrivenPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_pipeline_m_time(&mut self) -> core::ffi::c_ulong;
    fn set_release_data_flag(
        &mut self,
        port: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_release_data_flag(&mut self, port: core::ffi::c_int) -> core::ffi::c_int;
    fn update_pipeline_m_time(&mut self) -> core::ffi::c_int;
    fn update_data_object(&mut self) -> core::ffi::c_int;
    fn update_data(&mut self, outputPort: core::ffi::c_int) -> core::ffi::c_int;
    fn request_data_object(&mut self) -> *mut core::ffi::c_void;
    fn request_information(&mut self) -> *mut core::ffi::c_void;
    fn request_data(&mut self) -> *mut core::ffi::c_void;
    fn request_data_not_generated(&mut self) -> *mut core::ffi::c_void;
    fn release_data(&mut self) -> *mut core::ffi::c_void;
    fn data_not_generated(&mut self) -> *mut core::ffi::c_void;
    fn new_data_object(&mut self, type_: &str) -> *mut core::ffi::c_void;
}
pub trait VtkDirectedGraphAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkEnsembleSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn add_member(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn remove_all_members(&mut self) -> ();
    fn get_number_of_members(&mut self) -> core::ffi::c_uint;
    fn set_current_member(&mut self, _arg: core::ffi::c_uint) -> ();
    fn get_current_member(&mut self) -> core::ffi::c_uint;
    fn set_meta_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn meta_data(&mut self) -> *mut core::ffi::c_void;
    fn update_member(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkExecutive {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_algorithm(&mut self) -> *mut core::ffi::c_void;
    fn update_information(&mut self) -> core::ffi::c_int;
    fn update(&mut self) -> core::ffi::c_int;
    fn get_number_of_input_ports(&mut self) -> core::ffi::c_int;
    fn get_number_of_output_ports(&mut self) -> core::ffi::c_int;
    fn get_number_of_input_connections(
        &mut self,
        port: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_output_information(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_input_information(
        &mut self,
        port: core::ffi::c_int,
        connection: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_input_executive(
        &mut self,
        port: core::ffi::c_int,
        connection: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_output_data(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn set_output_data(
        &mut self,
        port: core::ffi::c_int,
        p1: *mut core::ffi::c_void,
        info: *mut core::ffi::c_void,
    ) -> ();
    fn get_input_data(
        &mut self,
        port: core::ffi::c_int,
        connection: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_shared_output_information(
        &mut self,
        outInfoVec: *mut core::ffi::c_void,
    ) -> ();
    fn register(&mut self, o: *mut core::ffi::c_void) -> ();
    fn producer(&mut self) -> *mut core::ffi::c_void;
    fn consumers(&mut self) -> *mut core::ffi::c_void;
    fn from_output_port(&mut self) -> *mut core::ffi::c_void;
    fn algorithm_before_forward(&mut self) -> *mut core::ffi::c_void;
    fn algorithm_after_forward(&mut self) -> *mut core::ffi::c_void;
    fn algorithm_direction(&mut self) -> *mut core::ffi::c_void;
    fn forward_direction(&mut self) -> *mut core::ffi::c_void;
    fn keys_to_copy(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkExplicitStructuredGridAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_explicit_structured_grid_input(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkExtentRCBPartitioner {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_number_of_partitions(&mut self, N: core::ffi::c_int) -> ();
    fn set_global_extent(
        &mut self,
        imin: core::ffi::c_int,
        imax: core::ffi::c_int,
        jmin: core::ffi::c_int,
        jmax: core::ffi::c_int,
        kmin: core::ffi::c_int,
        kmax: core::ffi::c_int,
    ) -> ();
    fn set_duplicate_nodes(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_duplicate_nodes(&mut self) -> core::ffi::c_int;
    fn duplicate_nodes_on(&mut self) -> ();
    fn duplicate_nodes_off(&mut self) -> ();
    fn set_number_of_ghost_layers(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_ghost_layers(&mut self) -> core::ffi::c_int;
    fn get_num_extents(&mut self) -> core::ffi::c_int;
    fn partition(&mut self) -> ();
}
pub trait VtkExtentSplitter {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn add_extent_source(
        &mut self,
        id: core::ffi::c_int,
        priority: core::ffi::c_int,
        x0: core::ffi::c_int,
        x1: core::ffi::c_int,
        y0: core::ffi::c_int,
        y1: core::ffi::c_int,
        z0: core::ffi::c_int,
        z1: core::ffi::c_int,
    ) -> ();
    fn remove_extent_source(&mut self, id: core::ffi::c_int) -> ();
    fn remove_all_extent_sources(&mut self) -> ();
    fn add_extent(
        &mut self,
        x0: core::ffi::c_int,
        x1: core::ffi::c_int,
        y0: core::ffi::c_int,
        y1: core::ffi::c_int,
        z0: core::ffi::c_int,
        z1: core::ffi::c_int,
    ) -> ();
    fn compute_sub_extents(&mut self) -> core::ffi::c_int;
    fn get_number_of_sub_extents(&mut self) -> core::ffi::c_int;
    fn get_sub_extent_source(&mut self, index: core::ffi::c_int) -> core::ffi::c_int;
    fn get_point_mode(&mut self) -> core::ffi::c_int;
    fn set_point_mode(&mut self, _arg: core::ffi::c_int) -> ();
    fn point_mode_on(&mut self) -> ();
    fn point_mode_off(&mut self) -> ();
}
pub trait VtkExtentTranslator {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_whole_extent(
        &mut self,
        _arg1: core::ffi::c_int,
        _arg2: core::ffi::c_int,
        _arg3: core::ffi::c_int,
        _arg4: core::ffi::c_int,
        _arg5: core::ffi::c_int,
        _arg6: core::ffi::c_int,
    ) -> ();
    fn set_extent(
        &mut self,
        _arg1: core::ffi::c_int,
        _arg2: core::ffi::c_int,
        _arg3: core::ffi::c_int,
        _arg4: core::ffi::c_int,
        _arg5: core::ffi::c_int,
        _arg6: core::ffi::c_int,
    ) -> ();
    fn set_piece(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_piece(&mut self) -> core::ffi::c_int;
    fn set_number_of_pieces(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_pieces(&mut self) -> core::ffi::c_int;
    fn set_ghost_level(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_ghost_level(&mut self) -> core::ffi::c_int;
    fn piece_to_extent(&mut self) -> core::ffi::c_int;
    fn piece_to_extent_by_points(&mut self) -> core::ffi::c_int;
    fn set_split_mode_to_block(&mut self) -> ();
    fn set_split_mode_to_x_slab(&mut self) -> ();
    fn set_split_mode_to_y_slab(&mut self) -> ();
    fn set_split_mode_to_z_slab(&mut self) -> ();
    fn get_split_mode(&mut self) -> core::ffi::c_int;
    fn update_split_mode(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkFilteringInformationKeyManager {
    fn register(&mut self, key: *mut core::ffi::c_void) -> ();
}
pub trait VtkGraphAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkHierarchicalBoxDataSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkHyperTreeGridAlgorithm {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_hyper_tree_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void;
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkImageAlgorithm {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn get_image_data_input(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkImageInPlaceFilter {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkImageProgressIterator {
    fn next_span(&mut self) -> ();
    fn is_at_end(&mut self) -> core::ffi::c_int;
}
pub trait VtkImageToStructuredGrid {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkImageToStructuredPoints {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_vector_input_data(&mut self, input: *mut core::ffi::c_void) -> ();
    fn get_vector_input(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_points_output(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkInformationDataObjectMetaDataKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn copy_default_information(
        &mut self,
        request: *mut core::ffi::c_void,
        fromInfo: *mut core::ffi::c_void,
        toInfo: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationExecutivePortKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn set(
        &mut self,
        info: *mut core::ffi::c_void,
        p1: *mut core::ffi::c_void,
        p2: core::ffi::c_int,
    ) -> ();
    fn get_executive(&mut self, info: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn get_port(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn get(
        &mut self,
        info: *mut core::ffi::c_void,
        executive: *mut core::ffi::c_void,
        port: &mut core::ffi::c_int,
    ) -> ();
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationExecutivePortVectorKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
    fn append(
        &mut self,
        info: *mut core::ffi::c_void,
        executive: *mut core::ffi::c_void,
        port: core::ffi::c_int,
    ) -> ();
    fn remove(
        &mut self,
        info: *mut core::ffi::c_void,
        executive: *mut core::ffi::c_void,
        port: core::ffi::c_int,
    ) -> ();
    fn length(&mut self, info: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn shallow_copy(
        &mut self,
        from: *mut core::ffi::c_void,
        to: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkInformationIntegerRequestKey {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_key(&mut self, name: &str, location: &str) -> *mut core::ffi::c_void;
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
pub trait VtkMoleculeAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_molecule_input(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkMultiBlockDataSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkMultiTimeStepAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkNonOverlappingAMRAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkOverlappingAMRAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkParallelReader {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn add_file_name(&mut self, fname: &str) -> ();
    fn clear_file_names(&mut self) -> ();
    fn get_number_of_file_names(&mut self) -> core::ffi::c_int;
    fn get_file_name(&mut self, i: core::ffi::c_int) -> &str;
    fn get_current_file_name(&mut self) -> &str;
    fn read_meta_data(&mut self, metadata: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn read_mesh(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_points(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_arrays(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}
pub trait VtkPartitionedDataSetAlgorithm {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkPartitionedDataSetCollectionAlgorithm {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkPassInputTypeAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_points_output(&mut self) -> *mut core::ffi::c_void;
    fn get_image_data_output(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_rectilinear_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_graph_output(&mut self) -> *mut core::ffi::c_void;
    fn get_molecule_output(&mut self) -> *mut core::ffi::c_void;
    fn get_table_output(&mut self) -> *mut core::ffi::c_void;
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkPiecewiseFunctionAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkPiecewiseFunctionShiftScale {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_position_shift(&mut self, _arg: core::ffi::c_double) -> ();
    fn set_position_scale(&mut self, _arg: core::ffi::c_double) -> ();
    fn set_value_shift(&mut self, _arg: core::ffi::c_double) -> ();
    fn set_value_scale(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_position_shift(&mut self) -> core::ffi::c_double;
    fn get_position_scale(&mut self) -> core::ffi::c_double;
    fn get_value_shift(&mut self) -> core::ffi::c_double;
    fn get_value_scale(&mut self) -> core::ffi::c_double;
}
pub trait VtkPointSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkPolyDataAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_poly_data_input(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkProgressObserver {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn update_progress(&mut self, amount: core::ffi::c_double) -> ();
    fn get_progress(&mut self) -> core::ffi::c_double;
}
pub trait VtkReaderAlgorithm {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn create_output(
        &mut self,
        currentOutput: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn read_meta_data(&mut self, metadata: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn read_time_dependent_meta_data(
        &mut self,
        p0: core::ffi::c_int,
        p1: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_mesh(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_points(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_arrays(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}
pub trait VtkReaderExecutive {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkRectilinearGridAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_rectilinear_grid_input(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkSMPProgressObserver {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn update_progress(&mut self, progress: core::ffi::c_double) -> ();
    fn get_local_observer(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkScalarTree {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn shallow_copy(&mut self, stree: *mut core::ffi::c_void) -> ();
    fn set_data_set(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_data_set(&mut self) -> *mut core::ffi::c_void;
    fn set_scalars(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_scalars(&mut self) -> *mut core::ffi::c_void;
    fn build_tree(&mut self) -> ();
    fn initialize(&mut self) -> ();
    fn init_traversal(&mut self, scalarValue: core::ffi::c_double) -> ();
    fn get_next_cell(
        &mut self,
        cellId: &mut core::ffi::c_longlong,
        ptIds: *mut core::ffi::c_void,
        cellScalars: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_scalar_value(&mut self) -> core::ffi::c_double;
    fn get_number_of_cell_batches(
        &mut self,
        scalarValue: core::ffi::c_double,
    ) -> core::ffi::c_longlong;
}
pub trait VtkSelectionAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkSimpleImageToImageFilter {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkSimpleReader {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn add_file_name(&mut self, fname: &str) -> ();
    fn clear_file_names(&mut self) -> ();
    fn get_number_of_file_names(&mut self) -> core::ffi::c_int;
    fn get_file_name(&mut self, i: core::ffi::c_int) -> &str;
    fn get_current_file_name(&mut self) -> &str;
    fn read_time_dependent_meta_data(
        &mut self,
        timestep: core::ffi::c_int,
        metadata: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_meta_data(&mut self, metadata: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn read_mesh(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_points(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_arrays(
        &mut self,
        piece: core::ffi::c_int,
        npieces: core::ffi::c_int,
        nghosts: core::ffi::c_int,
        timestep: core::ffi::c_int,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn get_time_value(&mut self, fname: &str) -> core::ffi::c_double;
    fn read_meta_data_simple(
        &mut self,
        p0: &str,
        p1: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_mesh_simple(
        &mut self,
        fname: &str,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_points_simple(
        &mut self,
        fname: &str,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn read_arrays_simple(
        &mut self,
        fname: &str,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}
pub trait VtkSimpleScalarTree {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_branching_factor(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_branching_factor_min_value(&mut self) -> core::ffi::c_int;
    fn get_branching_factor_max_value(&mut self) -> core::ffi::c_int;
    fn get_branching_factor(&mut self) -> core::ffi::c_int;
    fn get_level(&mut self) -> core::ffi::c_int;
    fn set_max_level(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_max_level_min_value(&mut self) -> core::ffi::c_int;
    fn get_max_level_max_value(&mut self) -> core::ffi::c_int;
    fn get_max_level(&mut self) -> core::ffi::c_int;
    fn build_tree(&mut self) -> ();
    fn initialize(&mut self) -> ();
    fn init_traversal(&mut self, scalarValue: core::ffi::c_double) -> ();
    fn get_next_cell(
        &mut self,
        cellId: &mut core::ffi::c_longlong,
        ptIds: *mut core::ffi::c_void,
        cellScalars: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_number_of_cell_batches(
        &mut self,
        scalarValue: core::ffi::c_double,
    ) -> core::ffi::c_longlong;
}
pub trait VtkSpanSpace {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_scalar_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> ();
    fn set_compute_scalar_range(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_compute_scalar_range(&mut self) -> core::ffi::c_int;
    fn compute_scalar_range_on(&mut self) -> ();
    fn compute_scalar_range_off(&mut self) -> ();
    fn set_resolution(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_resolution_min_value(&mut self) -> core::ffi::c_longlong;
    fn get_resolution_max_value(&mut self) -> core::ffi::c_longlong;
    fn get_resolution(&mut self) -> core::ffi::c_longlong;
    fn set_compute_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_compute_resolution(&mut self) -> core::ffi::c_int;
    fn compute_resolution_on(&mut self) -> ();
    fn compute_resolution_off(&mut self) -> ();
    fn set_number_of_cells_per_bucket(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_cells_per_bucket_min_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_cells_per_bucket_max_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_cells_per_bucket(&mut self) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn build_tree(&mut self) -> ();
    fn init_traversal(&mut self, scalarValue: core::ffi::c_double) -> ();
    fn get_next_cell(
        &mut self,
        cellId: &mut core::ffi::c_longlong,
        ptIds: *mut core::ffi::c_void,
        cellScalars: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn get_number_of_cell_batches(
        &mut self,
        scalarValue: core::ffi::c_double,
    ) -> core::ffi::c_longlong;
    fn set_batch_size(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_batch_size_min_value(&mut self) -> core::ffi::c_longlong;
    fn get_batch_size_max_value(&mut self) -> core::ffi::c_longlong;
    fn get_batch_size(&mut self) -> core::ffi::c_longlong;
}
pub trait VtkSphereTree {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_data_set(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_data_set(&mut self) -> *mut core::ffi::c_void;
    fn build(&mut self) -> ();
    fn set_build_hierarchy(&mut self, _arg: bool) -> ();
    fn get_build_hierarchy(&mut self) -> bool;
    fn build_hierarchy_on(&mut self) -> ();
    fn build_hierarchy_off(&mut self) -> ();
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_resolution(&mut self) -> core::ffi::c_int;
    fn set_max_level(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_max_level_min_value(&mut self) -> core::ffi::c_int;
    fn get_max_level_max_value(&mut self) -> core::ffi::c_int;
    fn get_max_level(&mut self) -> core::ffi::c_int;
    fn get_number_of_levels(&mut self) -> core::ffi::c_int;
}
pub trait VtkStreamingDemandDrivenPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn update(&mut self) -> core::ffi::c_int;
    fn update_whole_extent(&mut self) -> core::ffi::c_int;
    fn propagate_update_extent(
        &mut self,
        outputPort: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn propagate_time(&mut self, outputPort: core::ffi::c_int) -> core::ffi::c_int;
    fn update_time_dependent_information(
        &mut self,
        outputPort: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn set_request_exact_extent(
        &mut self,
        port: core::ffi::c_int,
        flag: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn get_request_exact_extent(&mut self, port: core::ffi::c_int) -> core::ffi::c_int;
    fn request_update_extent(&mut self) -> *mut core::ffi::c_void;
    fn request_update_time(&mut self) -> *mut core::ffi::c_void;
    fn request_time_dependent_information(&mut self) -> *mut core::ffi::c_void;
    fn continue_executing(&mut self) -> *mut core::ffi::c_void;
    fn update_extent_initialized(&mut self) -> *mut core::ffi::c_void;
    fn update_extent(&mut self) -> *mut core::ffi::c_void;
    fn update_piece_number(&mut self) -> *mut core::ffi::c_void;
    fn update_number_of_pieces(&mut self) -> *mut core::ffi::c_void;
    fn update_number_of_ghost_levels(&mut self) -> *mut core::ffi::c_void;
    fn combined_update_extent(&mut self) -> *mut core::ffi::c_void;
    fn whole_extent(&mut self) -> *mut core::ffi::c_void;
    fn unrestricted_update_extent(&mut self) -> *mut core::ffi::c_void;
    fn exact_extent(&mut self) -> *mut core::ffi::c_void;
    fn time_steps(&mut self) -> *mut core::ffi::c_void;
    fn time_range(&mut self) -> *mut core::ffi::c_void;
    fn update_time_step(&mut self) -> *mut core::ffi::c_void;
    fn time_dependent_information(&mut self) -> *mut core::ffi::c_void;
    fn bounds(&mut self) -> *mut core::ffi::c_void;
    fn get_update_piece(&mut self, p0: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn get_update_number_of_pieces(
        &mut self,
        p0: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn get_update_ghost_level(&mut self, p0: *mut core::ffi::c_void) -> core::ffi::c_int;
}
pub trait VtkStructuredGridAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_grid_input(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkTableAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkThreadedCompositeDataPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkThreadedImageAlgorithm {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_enable_smp(&mut self) -> bool;
    fn set_enable_smp(&mut self, _arg: bool) -> ();
    fn set_global_default_enable_smp(&mut self, enable: bool) -> ();
    fn get_global_default_enable_smp(&mut self) -> bool;
    fn set_minimum_piece_size(
        &mut self,
        _arg1: core::ffi::c_int,
        _arg2: core::ffi::c_int,
        _arg3: core::ffi::c_int,
    ) -> ();
    fn set_desired_bytes_per_piece(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_desired_bytes_per_piece(&mut self) -> core::ffi::c_longlong;
    fn set_split_mode(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_split_mode_min_value(&mut self) -> core::ffi::c_int;
    fn get_split_mode_max_value(&mut self) -> core::ffi::c_int;
    fn set_split_mode_to_slab(&mut self) -> ();
    fn set_split_mode_to_beam(&mut self) -> ();
    fn set_split_mode_to_block(&mut self) -> ();
    fn get_split_mode(&mut self) -> core::ffi::c_int;
    fn set_number_of_threads(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_threads_min_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_threads_max_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_threads(&mut self) -> core::ffi::c_int;
}
pub trait VtkTreeAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkTrivialConsumer {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkTrivialProducer {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, output: *mut core::ffi::c_void) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn set_whole_extent(
        &mut self,
        _arg1: core::ffi::c_int,
        _arg2: core::ffi::c_int,
        _arg3: core::ffi::c_int,
        _arg4: core::ffi::c_int,
        _arg5: core::ffi::c_int,
        _arg6: core::ffi::c_int,
    ) -> ();
    fn fill_output_data_information(
        &mut self,
        output: *mut core::ffi::c_void,
        outInfo: *mut core::ffi::c_void,
    ) -> ();
}
pub trait VtkUndirectedGraphAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> ();
}
pub trait VtkUniformGridAMRAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkUniformGridPartitioner {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_partitions(&mut self) -> core::ffi::c_int;
    fn set_number_of_partitions(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_ghost_layers(&mut self) -> core::ffi::c_int;
    fn set_number_of_ghost_layers(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_duplicate_nodes(&mut self) -> core::ffi::c_int;
    fn set_duplicate_nodes(&mut self, _arg: core::ffi::c_int) -> ();
    fn duplicate_nodes_on(&mut self) -> ();
    fn duplicate_nodes_off(&mut self) -> ();
}
pub trait VtkUnstructuredGridAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn get_unstructured_grid_input(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
pub trait VtkUnstructuredGridBaseAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_output(&mut self) -> *mut core::ffi::c_void;
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> ();
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> ();
}
impl VtkAlgorithm for vtkAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_new_instance(self.0) }
    }
    fn has_executive(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_has_executive(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_has_executive(self.0) }
    }
    fn get_executive(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_executive(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_executive(self.0) }
    }
    fn set_executive(&mut self, executive: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_executive(
                sself: *mut core::ffi::c_void,
                executive: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_set_executive(self.0, executive) }
    }
    fn process_request(
        &mut self,
        request: *mut core::ffi::c_void,
        inInfo: *mut core::ffi::c_void,
        outInfo: *mut core::ffi::c_void,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_process_request(
                sself: *mut core::ffi::c_void,
                request: *mut core::ffi::c_void,
                inInfo: *mut core::ffi::c_void,
                outInfo: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_process_request(self.0, request, inInfo, outInfo) }
    }
    fn modify_request(
        &mut self,
        request: *mut core::ffi::c_void,
        when: core::ffi::c_int,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_modify_request(
                sself: *mut core::ffi::c_void,
                request: *mut core::ffi::c_void,
                when: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_modify_request(self.0, request, when) }
    }
    fn get_input_port_information(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_input_port_information(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_input_port_information(self.0, port) }
    }
    fn get_output_port_information(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_output_port_information(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_output_port_information(self.0, port) }
    }
    fn get_information(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_information(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_information(self.0) }
    }
    fn set_information(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_information(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_set_information(self.0, p0) }
    }
    fn get_number_of_input_ports(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_number_of_input_ports(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_number_of_input_ports(self.0) }
    }
    fn get_number_of_output_ports(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_number_of_output_ports(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_number_of_output_ports(self.0) }
    }
    fn register(&mut self, o: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_register(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_register(self.0, o) }
    }
    fn set_abort_execute(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_abort_execute(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_algorithm_set_abort_execute(self.0, _arg) }
    }
    fn get_abort_execute(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_abort_execute(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_abort_execute(self.0) }
    }
    fn abort_execute_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_abort_execute_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_abort_execute_on(self.0) }
    }
    fn abort_execute_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_abort_execute_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_abort_execute_off(self.0) }
    }
    fn get_progress(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_algorithm_get_progress(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_algorithm_get_progress(self.0) }
    }
    fn set_progress(&mut self, p0: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_progress(
                sself: *mut core::ffi::c_void,
                p0: core::ffi::c_double,
            );
        }
        unsafe { vtk_algorithm_set_progress(self.0, p0) }
    }
    fn update_progress(&mut self, amount: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_update_progress(
                sself: *mut core::ffi::c_void,
                amount: core::ffi::c_double,
            );
        }
        unsafe { vtk_algorithm_update_progress(self.0, amount) }
    }
    fn set_progress_shift_scale(
        &mut self,
        shift: core::ffi::c_double,
        scale: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_progress_shift_scale(
                sself: *mut core::ffi::c_void,
                shift: core::ffi::c_double,
                scale: core::ffi::c_double,
            );
        }
        unsafe { vtk_algorithm_set_progress_shift_scale(self.0, shift, scale) }
    }
    fn get_progress_shift(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_algorithm_get_progress_shift(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_algorithm_get_progress_shift(self.0) }
    }
    fn get_progress_scale(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_algorithm_get_progress_scale(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_algorithm_get_progress_scale(self.0) }
    }
    fn set_progress_text(&mut self, ptext: &str) -> () {
        let c_ptext = std::ffi::CString::new(ptext).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_algorithm_set_progress_text(
                sself: *mut core::ffi::c_void,
                ptext: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_algorithm_set_progress_text(self.0, c_ptext.as_ptr()) }
    }
    fn get_error_code(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_algorithm_get_error_code(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_algorithm_get_error_code(self.0) }
    }
    fn input_is_optional(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_input_is_optional(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_input_is_optional(self.0) }
    }
    fn input_is_repeatable(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_input_is_repeatable(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_input_is_repeatable(self.0) }
    }
    fn input_required_fields(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_input_required_fields(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_input_required_fields(self.0) }
    }
    fn input_required_data_type(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_input_required_data_type(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_input_required_data_type(self.0) }
    }
    fn input_arrays_to_process(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_input_arrays_to_process(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_input_arrays_to_process(self.0) }
    }
    fn input_port(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_input_port(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_input_port(self.0) }
    }
    fn input_connection(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_input_connection(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_input_connection(self.0) }
    }
    fn can_produce_sub_extent(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_can_produce_sub_extent(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_can_produce_sub_extent(self.0) }
    }
    fn can_handle_piece_request(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_can_handle_piece_request(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_can_handle_piece_request(self.0) }
    }
    fn set_input_array_to_process(
        &mut self,
        idx: core::ffi::c_int,
        port: core::ffi::c_int,
        connection: core::ffi::c_int,
        fieldAssociation: core::ffi::c_int,
        name: &str,
    ) -> () {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_algorithm_set_input_array_to_process(
                sself: *mut core::ffi::c_void,
                idx: core::ffi::c_int,
                port: core::ffi::c_int,
                connection: core::ffi::c_int,
                fieldAssociation: core::ffi::c_int,
                name: *const core::ffi::c_char,
            );
        }
        unsafe {
            vtk_algorithm_set_input_array_to_process(
                self.0,
                idx,
                port,
                connection,
                fieldAssociation,
                c_name.as_ptr(),
            )
        }
    }
    fn get_input_array_information(
        &mut self,
        idx: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_input_array_information(
                sself: *mut core::ffi::c_void,
                idx: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_input_array_information(self.0, idx) }
    }
    fn remove_all_inputs(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_remove_all_inputs(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_remove_all_inputs(self.0) }
    }
    fn get_output_data_object(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_output_data_object(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_output_data_object(self.0, port) }
    }
    fn get_input_data_object(
        &mut self,
        port: core::ffi::c_int,
        connection: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_input_data_object(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                connection: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_input_data_object(self.0, port, connection) }
    }
    fn set_input_connection(
        &mut self,
        port: core::ffi::c_int,
        input: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_input_connection(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                input: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_set_input_connection(self.0, port, input) }
    }
    fn add_input_connection(
        &mut self,
        port: core::ffi::c_int,
        input: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_add_input_connection(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                input: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_add_input_connection(self.0, port, input) }
    }
    fn remove_input_connection(
        &mut self,
        port: core::ffi::c_int,
        input: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_remove_input_connection(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                input: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_remove_input_connection(self.0, port, input) }
    }
    fn remove_all_input_connections(&mut self, port: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_remove_all_input_connections(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            );
        }
        unsafe { vtk_algorithm_remove_all_input_connections(self.0, port) }
    }
    fn set_input_data_object(
        &mut self,
        port: core::ffi::c_int,
        data: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_input_data_object(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                data: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_set_input_data_object(self.0, port, data) }
    }
    fn add_input_data_object(
        &mut self,
        port: core::ffi::c_int,
        data: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_add_input_data_object(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                data: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_add_input_data_object(self.0, port, data) }
    }
    fn get_output_port(&mut self, index: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_output_port(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_output_port(self.0, index) }
    }
    fn get_number_of_input_connections(
        &mut self,
        port: core::ffi::c_int,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_number_of_input_connections(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_number_of_input_connections(self.0, port) }
    }
    fn get_total_number_of_input_connections(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_total_number_of_input_connections(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_total_number_of_input_connections(self.0) }
    }
    fn get_input_connection(
        &mut self,
        port: core::ffi::c_int,
        index: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_input_connection(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                index: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_input_connection(self.0, port, index) }
    }
    fn get_input_algorithm(
        &mut self,
        port: core::ffi::c_int,
        index: core::ffi::c_int,
        algPort: &mut core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_input_algorithm(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                index: core::ffi::c_int,
                algPort: &mut core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_input_algorithm(self.0, port, index, algPort) }
    }
    fn get_input_executive(
        &mut self,
        port: core::ffi::c_int,
        index: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_input_executive(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                index: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_input_executive(self.0, port, index) }
    }
    fn get_input_information(
        &mut self,
        port: core::ffi::c_int,
        index: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_input_information(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                index: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_input_information(self.0, port, index) }
    }
    fn get_output_information(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_output_information(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_output_information(self.0, port) }
    }
    fn update(&mut self, port: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_update(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            );
        }
        unsafe { vtk_algorithm_update(self.0, port) }
    }
    fn update_information(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_update_information(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_update_information(self.0) }
    }
    fn update_data_object(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_update_data_object(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_update_data_object(self.0) }
    }
    fn propagate_update_extent(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_propagate_update_extent(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_propagate_update_extent(self.0) }
    }
    fn update_whole_extent(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_update_whole_extent(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_update_whole_extent(self.0) }
    }
    fn convert_total_input_to_port_connection(
        &mut self,
        ind: core::ffi::c_int,
        port: &mut core::ffi::c_int,
        conn: &mut core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_convert_total_input_to_port_connection(
                sself: *mut core::ffi::c_void,
                ind: core::ffi::c_int,
                port: &mut core::ffi::c_int,
                conn: &mut core::ffi::c_int,
            );
        }
        unsafe {
            vtk_algorithm_convert_total_input_to_port_connection(self.0, ind, port, conn)
        }
    }
    fn set_release_data_flag(&mut self, p0: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_release_data_flag(
                sself: *mut core::ffi::c_void,
                p0: core::ffi::c_int,
            );
        }
        unsafe { vtk_algorithm_set_release_data_flag(self.0, p0) }
    }
    fn get_release_data_flag(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_release_data_flag(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_release_data_flag(self.0) }
    }
    fn release_data_flag_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_release_data_flag_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_release_data_flag_on(self.0) }
    }
    fn release_data_flag_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_release_data_flag_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_algorithm_release_data_flag_off(self.0) }
    }
    fn update_extent_is_empty(
        &mut self,
        pinfo: *mut core::ffi::c_void,
        output: *mut core::ffi::c_void,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_update_extent_is_empty(
                sself: *mut core::ffi::c_void,
                pinfo: *mut core::ffi::c_void,
                output: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_update_extent_is_empty(self.0, pinfo, output) }
    }
    fn set_default_executive_prototype(&mut self, proto: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_default_executive_prototype(
                sself: *mut core::ffi::c_void,
                proto: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_set_default_executive_prototype(self.0, proto) }
    }
    fn get_update_piece(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_update_piece(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_update_piece(self.0) }
    }
    fn get_update_number_of_pieces(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_update_number_of_pieces(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_update_number_of_pieces(self.0) }
    }
    fn get_update_ghost_level(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_get_update_ghost_level(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_get_update_ghost_level(self.0) }
    }
    fn set_progress_observer(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_set_progress_observer(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_set_progress_observer(self.0, p0) }
    }
    fn get_progress_observer(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_get_progress_observer(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_get_progress_observer(self.0) }
    }
}
impl VtkAlgorithmOutput for vtkAlgorithmOutput {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_output_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_output_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_output_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_output_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_output_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_output_new_instance(self.0) }
    }
    fn set_index(&mut self, index: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_output_set_index(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
            );
        }
        unsafe { vtk_algorithm_output_set_index(self.0, index) }
    }
    fn get_index(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_algorithm_output_get_index(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_algorithm_output_get_index(self.0) }
    }
    fn get_producer(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_algorithm_output_get_producer(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_algorithm_output_get_producer(self.0) }
    }
    fn set_producer(&mut self, producer: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_algorithm_output_set_producer(
                sself: *mut core::ffi::c_void,
                producer: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_algorithm_output_set_producer(self.0, producer) }
    }
}
impl VtkAnnotationLayersAlgorithm for vtkAnnotationLayersAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_annotation_layers_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_annotation_layers_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_annotation_layers_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_annotation_layers_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_annotation_layers_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_annotation_layers_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_annotation_layers_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_annotation_layers_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_annotation_layers_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_annotation_layers_algorithm_set_input_data(self.0, obj) }
    }
}
impl VtkArrayDataAlgorithm for vtkArrayDataAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_array_data_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_array_data_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_array_data_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_array_data_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_array_data_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_array_data_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_array_data_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_array_data_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_array_data_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_array_data_algorithm_set_input_data(self.0, obj) }
    }
}
impl VtkCachedStreamingDemandDrivenPipeline for vtkCachedStreamingDemandDrivenPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cached_streaming_demand_driven_pipeline_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cached_streaming_demand_driven_pipeline_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cached_streaming_demand_driven_pipeline_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cached_streaming_demand_driven_pipeline_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cached_streaming_demand_driven_pipeline_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cached_streaming_demand_driven_pipeline_new_instance(self.0) }
    }
    fn set_cache_size(&mut self, size: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cached_streaming_demand_driven_pipeline_set_cache_size(
                sself: *mut core::ffi::c_void,
                size: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_cached_streaming_demand_driven_pipeline_set_cache_size(self.0, size)
        }
    }
    fn get_cache_size(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cached_streaming_demand_driven_pipeline_get_cache_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cached_streaming_demand_driven_pipeline_get_cache_size(self.0) }
    }
}
impl VtkCastToConcrete for vtkCastToConcrete {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cast_to_concrete_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cast_to_concrete_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cast_to_concrete_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cast_to_concrete_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cast_to_concrete_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cast_to_concrete_new_instance(self.0) }
    }
}
impl VtkCompositeDataPipeline for vtkCompositeDataPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_pipeline_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_pipeline_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_pipeline_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_pipeline_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_pipeline_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_pipeline_new_instance(self.0) }
    }
    fn get_composite_output_data(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_pipeline_get_composite_output_data(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_pipeline_get_composite_output_data(self.0, port) }
    }
    fn load_requested_blocks(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_pipeline_load_requested_blocks(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_pipeline_load_requested_blocks(self.0) }
    }
    fn composite_data_meta_data(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_pipeline_composite_data_meta_data(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_pipeline_composite_data_meta_data(self.0) }
    }
    fn update_composite_indices(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_pipeline_update_composite_indices(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_pipeline_update_composite_indices(self.0) }
    }
    fn block_amount_of_detail(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_pipeline_block_amount_of_detail(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_pipeline_block_amount_of_detail(self.0) }
    }
}
impl VtkCompositeDataSetAlgorithm for vtkCompositeDataSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_set_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_set_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_set_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_set_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_set_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_set_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_composite_data_set_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_composite_data_set_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_composite_data_set_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_composite_data_set_algorithm_set_input_data(self.0, p0) }
    }
}
impl VtkDataObjectAlgorithm for vtkDataObjectAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_object_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_object_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_object_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_object_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_object_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_object_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_object_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_object_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_object_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_object_algorithm_set_output(self.0, d) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_object_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_object_algorithm_get_input(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_object_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_object_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_object_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_object_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkDataSetAlgorithm for vtkDataSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_get_output(self.0) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_get_input(self.0) }
    }
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_get_poly_data_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_get_poly_data_output(self.0) }
    }
    fn get_structured_points_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_get_structured_points_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_get_structured_points_output(self.0) }
    }
    fn get_image_data_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_get_image_data_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_get_image_data_output(self.0) }
    }
    fn get_structured_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_get_structured_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_get_structured_grid_output(self.0) }
    }
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_get_unstructured_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_get_unstructured_grid_output(self.0) }
    }
    fn get_rectilinear_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_get_rectilinear_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_data_set_algorithm_get_rectilinear_grid_output(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_set_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_data_set_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_data_set_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkDemandDrivenPipeline for vtkDemandDrivenPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_new_instance(self.0) }
    }
    fn get_pipeline_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_get_pipeline_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_demand_driven_pipeline_get_pipeline_m_time(self.0) }
    }
    fn set_release_data_flag(
        &mut self,
        port: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_set_release_data_flag(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                n: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_demand_driven_pipeline_set_release_data_flag(self.0, port, n) }
    }
    fn get_release_data_flag(&mut self, port: core::ffi::c_int) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_get_release_data_flag(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_demand_driven_pipeline_get_release_data_flag(self.0, port) }
    }
    fn update_pipeline_m_time(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_update_pipeline_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_demand_driven_pipeline_update_pipeline_m_time(self.0) }
    }
    fn update_data_object(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_update_data_object(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_demand_driven_pipeline_update_data_object(self.0) }
    }
    fn update_data(&mut self, outputPort: core::ffi::c_int) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_update_data(
                sself: *mut core::ffi::c_void,
                outputPort: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_demand_driven_pipeline_update_data(self.0, outputPort) }
    }
    fn request_data_object(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_request_data_object(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_request_data_object(self.0) }
    }
    fn request_information(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_request_information(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_request_information(self.0) }
    }
    fn request_data(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_request_data(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_request_data(self.0) }
    }
    fn request_data_not_generated(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_request_data_not_generated(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_request_data_not_generated(self.0) }
    }
    fn release_data(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_release_data(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_release_data(self.0) }
    }
    fn data_not_generated(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_data_not_generated(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_data_not_generated(self.0) }
    }
    fn new_data_object(&mut self, type_: &str) -> *mut core::ffi::c_void {
        let c_type = std::ffi::CString::new(type_).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_demand_driven_pipeline_new_data_object(
                sself: *mut core::ffi::c_void,
                type_: *const core::ffi::c_char,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_demand_driven_pipeline_new_data_object(self.0, c_type.as_ptr()) }
    }
}
impl VtkDirectedGraphAlgorithm for vtkDirectedGraphAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_directed_graph_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_directed_graph_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_directed_graph_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_directed_graph_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_directed_graph_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_directed_graph_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_directed_graph_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_directed_graph_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_directed_graph_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_directed_graph_algorithm_set_input_data(self.0, obj) }
    }
}
impl VtkEnsembleSource for vtkEnsembleSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_ensemble_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_ensemble_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_ensemble_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_ensemble_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_ensemble_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_ensemble_source_new_instance(self.0) }
    }
    fn add_member(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_ensemble_source_add_member(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_ensemble_source_add_member(self.0, p0) }
    }
    fn remove_all_members(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_ensemble_source_remove_all_members(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_ensemble_source_remove_all_members(self.0) }
    }
    fn get_number_of_members(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_ensemble_source_get_number_of_members(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_ensemble_source_get_number_of_members(self.0) }
    }
    fn set_current_member(&mut self, _arg: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_ensemble_source_set_current_member(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_uint,
            );
        }
        unsafe { vtk_ensemble_source_set_current_member(self.0, _arg) }
    }
    fn get_current_member(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_ensemble_source_get_current_member(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_ensemble_source_get_current_member(self.0) }
    }
    fn set_meta_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_ensemble_source_set_meta_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_ensemble_source_set_meta_data(self.0, p0) }
    }
    fn meta_data(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_ensemble_source_meta_data(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_ensemble_source_meta_data(self.0) }
    }
    fn update_member(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_ensemble_source_update_member(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_ensemble_source_update_member(self.0) }
    }
}
impl VtkExplicitStructuredGridAlgorithm for vtkExplicitStructuredGridAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_explicit_structured_grid_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_explicit_structured_grid_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_explicit_structured_grid_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_explicit_structured_grid_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_explicit_structured_grid_algorithm_set_output(self.0, d) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_explicit_structured_grid_algorithm_get_input(self.0) }
    }
    fn get_explicit_structured_grid_input(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_get_explicit_structured_grid_input(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_explicit_structured_grid_algorithm_get_explicit_structured_grid_input(
                self.0,
                port,
            )
        }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_explicit_structured_grid_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_explicit_structured_grid_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_explicit_structured_grid_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkExtentRCBPartitioner for vtkExtentRCBPartitioner {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_rcb_partitioner_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_rcb_partitioner_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_rcb_partitioner_new_instance(self.0) }
    }
    fn set_number_of_partitions(&mut self, N: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_set_number_of_partitions(
                sself: *mut core::ffi::c_void,
                N: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_rcb_partitioner_set_number_of_partitions(self.0, N) }
    }
    fn set_global_extent(
        &mut self,
        imin: core::ffi::c_int,
        imax: core::ffi::c_int,
        jmin: core::ffi::c_int,
        jmax: core::ffi::c_int,
        kmin: core::ffi::c_int,
        kmax: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_set_global_extent(
                sself: *mut core::ffi::c_void,
                imin: core::ffi::c_int,
                imax: core::ffi::c_int,
                jmin: core::ffi::c_int,
                jmax: core::ffi::c_int,
                kmin: core::ffi::c_int,
                kmax: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_extent_rcb_partitioner_set_global_extent(
                self.0,
                imin,
                imax,
                jmin,
                jmax,
                kmin,
                kmax,
            )
        }
    }
    fn set_duplicate_nodes(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_set_duplicate_nodes(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_rcb_partitioner_set_duplicate_nodes(self.0, _arg) }
    }
    fn get_duplicate_nodes(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_get_duplicate_nodes(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_rcb_partitioner_get_duplicate_nodes(self.0) }
    }
    fn duplicate_nodes_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_duplicate_nodes_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_extent_rcb_partitioner_duplicate_nodes_on(self.0) }
    }
    fn duplicate_nodes_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_duplicate_nodes_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_extent_rcb_partitioner_duplicate_nodes_off(self.0) }
    }
    fn set_number_of_ghost_layers(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_set_number_of_ghost_layers(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_rcb_partitioner_set_number_of_ghost_layers(self.0, _arg) }
    }
    fn get_number_of_ghost_layers(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_get_number_of_ghost_layers(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_rcb_partitioner_get_number_of_ghost_layers(self.0) }
    }
    fn get_num_extents(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_get_num_extents(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_rcb_partitioner_get_num_extents(self.0) }
    }
    fn partition(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_rcb_partitioner_partition(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_extent_rcb_partitioner_partition(self.0) }
    }
}
impl VtkExtentSplitter for vtkExtentSplitter {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_splitter_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_splitter_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_splitter_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_splitter_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_splitter_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_splitter_new(self.0) }
    }
    fn add_extent_source(
        &mut self,
        id: core::ffi::c_int,
        priority: core::ffi::c_int,
        x0: core::ffi::c_int,
        x1: core::ffi::c_int,
        y0: core::ffi::c_int,
        y1: core::ffi::c_int,
        z0: core::ffi::c_int,
        z1: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_extent_splitter_add_extent_source(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_int,
                priority: core::ffi::c_int,
                x0: core::ffi::c_int,
                x1: core::ffi::c_int,
                y0: core::ffi::c_int,
                y1: core::ffi::c_int,
                z0: core::ffi::c_int,
                z1: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_extent_splitter_add_extent_source(
                self.0,
                id,
                priority,
                x0,
                x1,
                y0,
                y1,
                z0,
                z1,
            )
        }
    }
    fn remove_extent_source(&mut self, id: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_extent_splitter_remove_extent_source(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_splitter_remove_extent_source(self.0, id) }
    }
    fn remove_all_extent_sources(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_splitter_remove_all_extent_sources(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_extent_splitter_remove_all_extent_sources(self.0) }
    }
    fn add_extent(
        &mut self,
        x0: core::ffi::c_int,
        x1: core::ffi::c_int,
        y0: core::ffi::c_int,
        y1: core::ffi::c_int,
        z0: core::ffi::c_int,
        z1: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_extent_splitter_add_extent(
                sself: *mut core::ffi::c_void,
                x0: core::ffi::c_int,
                x1: core::ffi::c_int,
                y0: core::ffi::c_int,
                y1: core::ffi::c_int,
                z0: core::ffi::c_int,
                z1: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_splitter_add_extent(self.0, x0, x1, y0, y1, z0, z1) }
    }
    fn compute_sub_extents(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_splitter_compute_sub_extents(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_splitter_compute_sub_extents(self.0) }
    }
    fn get_number_of_sub_extents(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_splitter_get_number_of_sub_extents(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_splitter_get_number_of_sub_extents(self.0) }
    }
    fn get_sub_extent_source(&mut self, index: core::ffi::c_int) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_splitter_get_sub_extent_source(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_splitter_get_sub_extent_source(self.0, index) }
    }
    fn get_point_mode(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_splitter_get_point_mode(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_splitter_get_point_mode(self.0) }
    }
    fn set_point_mode(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_extent_splitter_set_point_mode(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_splitter_set_point_mode(self.0, _arg) }
    }
    fn point_mode_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_splitter_point_mode_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_extent_splitter_point_mode_on(self.0) }
    }
    fn point_mode_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_splitter_point_mode_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_extent_splitter_point_mode_off(self.0) }
    }
}
impl VtkExtentTranslator for vtkExtentTranslator {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_translator_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_translator_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_translator_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_translator_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_translator_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_translator_new_instance(self.0) }
    }
    fn set_whole_extent(
        &mut self,
        _arg1: core::ffi::c_int,
        _arg2: core::ffi::c_int,
        _arg3: core::ffi::c_int,
        _arg4: core::ffi::c_int,
        _arg5: core::ffi::c_int,
        _arg6: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_whole_extent(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_int,
                _arg2: core::ffi::c_int,
                _arg3: core::ffi::c_int,
                _arg4: core::ffi::c_int,
                _arg5: core::ffi::c_int,
                _arg6: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_extent_translator_set_whole_extent(
                self.0,
                _arg1,
                _arg2,
                _arg3,
                _arg4,
                _arg5,
                _arg6,
            )
        }
    }
    fn set_extent(
        &mut self,
        _arg1: core::ffi::c_int,
        _arg2: core::ffi::c_int,
        _arg3: core::ffi::c_int,
        _arg4: core::ffi::c_int,
        _arg5: core::ffi::c_int,
        _arg6: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_extent(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_int,
                _arg2: core::ffi::c_int,
                _arg3: core::ffi::c_int,
                _arg4: core::ffi::c_int,
                _arg5: core::ffi::c_int,
                _arg6: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_extent_translator_set_extent(
                self.0,
                _arg1,
                _arg2,
                _arg3,
                _arg4,
                _arg5,
                _arg6,
            )
        }
    }
    fn set_piece(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_piece(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_translator_set_piece(self.0, _arg) }
    }
    fn get_piece(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_translator_get_piece(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_translator_get_piece(self.0) }
    }
    fn set_number_of_pieces(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_number_of_pieces(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_translator_set_number_of_pieces(self.0, _arg) }
    }
    fn get_number_of_pieces(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_translator_get_number_of_pieces(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_translator_get_number_of_pieces(self.0) }
    }
    fn set_ghost_level(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_ghost_level(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_extent_translator_set_ghost_level(self.0, _arg) }
    }
    fn get_ghost_level(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_translator_get_ghost_level(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_translator_get_ghost_level(self.0) }
    }
    fn piece_to_extent(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_translator_piece_to_extent(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_translator_piece_to_extent(self.0) }
    }
    fn piece_to_extent_by_points(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_translator_piece_to_extent_by_points(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_translator_piece_to_extent_by_points(self.0) }
    }
    fn set_split_mode_to_block(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_split_mode_to_block(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_extent_translator_set_split_mode_to_block(self.0) }
    }
    fn set_split_mode_to_x_slab(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_split_mode_to_x_slab(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_extent_translator_set_split_mode_to_x_slab(self.0) }
    }
    fn set_split_mode_to_y_slab(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_split_mode_to_y_slab(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_extent_translator_set_split_mode_to_y_slab(self.0) }
    }
    fn set_split_mode_to_z_slab(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_extent_translator_set_split_mode_to_z_slab(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_extent_translator_set_split_mode_to_z_slab(self.0) }
    }
    fn get_split_mode(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_extent_translator_get_split_mode(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_extent_translator_get_split_mode(self.0) }
    }
    fn update_split_mode(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_extent_translator_update_split_mode(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_extent_translator_update_split_mode(self.0) }
    }
}
impl VtkGraphAlgorithm for vtkGraphAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_graph_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_graph_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_graph_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_graph_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_graph_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_graph_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_graph_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_graph_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_graph_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_graph_algorithm_set_input_data(self.0, obj) }
    }
}
impl VtkHierarchicalBoxDataSetAlgorithm for vtkHierarchicalBoxDataSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hierarchical_box_data_set_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hierarchical_box_data_set_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hierarchical_box_data_set_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hierarchical_box_data_set_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hierarchical_box_data_set_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hierarchical_box_data_set_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hierarchical_box_data_set_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hierarchical_box_data_set_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_hierarchical_box_data_set_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hierarchical_box_data_set_algorithm_set_input_data(self.0, p0) }
    }
}
impl VtkImageToStructuredGrid for vtkImageToStructuredGrid {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_image_to_structured_grid_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_image_to_structured_grid_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_image_to_structured_grid_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_image_to_structured_grid_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_image_to_structured_grid_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_image_to_structured_grid_new_instance(self.0) }
    }
}
impl VtkImageToStructuredPoints for vtkImageToStructuredPoints {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_image_to_structured_points_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_image_to_structured_points_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_image_to_structured_points_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_image_to_structured_points_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_image_to_structured_points_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_image_to_structured_points_new_instance(self.0) }
    }
    fn set_vector_input_data(&mut self, input: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_image_to_structured_points_set_vector_input_data(
                sself: *mut core::ffi::c_void,
                input: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_image_to_structured_points_set_vector_input_data(self.0, input) }
    }
    fn get_vector_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_image_to_structured_points_get_vector_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_image_to_structured_points_get_vector_input(self.0) }
    }
    fn get_structured_points_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_image_to_structured_points_get_structured_points_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_image_to_structured_points_get_structured_points_output(self.0) }
    }
}
impl VtkMoleculeAlgorithm for vtkMoleculeAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_molecule_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_molecule_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_molecule_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_molecule_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_molecule_algorithm_set_output(self.0, d) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_molecule_algorithm_get_input(self.0) }
    }
    fn get_molecule_input(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_get_molecule_input(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_molecule_algorithm_get_molecule_input(self.0, port) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_molecule_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_molecule_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_molecule_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkMultiBlockDataSetAlgorithm for vtkMultiBlockDataSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_block_data_set_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_block_data_set_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_block_data_set_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_block_data_set_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_block_data_set_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_block_data_set_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_block_data_set_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_block_data_set_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_multi_block_data_set_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_multi_block_data_set_algorithm_set_input_data(self.0, p0) }
    }
}
impl VtkMultiTimeStepAlgorithm for vtkMultiTimeStepAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_time_step_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_time_step_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_time_step_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_time_step_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_multi_time_step_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_multi_time_step_algorithm_new_instance(self.0) }
    }
}
impl VtkNonOverlappingAMRAlgorithm for vtkNonOverlappingAMRAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_non_overlapping_amr_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_non_overlapping_amr_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_non_overlapping_amr_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_non_overlapping_amr_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_non_overlapping_amr_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_non_overlapping_amr_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_non_overlapping_amr_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_non_overlapping_amr_algorithm_get_output(self.0) }
    }
}
impl VtkOverlappingAMRAlgorithm for vtkOverlappingAMRAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_overlapping_amr_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_overlapping_amr_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_overlapping_amr_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_overlapping_amr_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_overlapping_amr_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_overlapping_amr_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_overlapping_amr_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_overlapping_amr_algorithm_get_output(self.0) }
    }
}
impl VtkPassInputTypeAlgorithm for vtkPassInputTypeAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_output(self.0) }
    }
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_poly_data_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_poly_data_output(self.0) }
    }
    fn get_structured_points_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_structured_points_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_structured_points_output(self.0) }
    }
    fn get_image_data_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_image_data_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_image_data_output(self.0) }
    }
    fn get_structured_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_structured_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_structured_grid_output(self.0) }
    }
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_unstructured_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_unstructured_grid_output(self.0) }
    }
    fn get_rectilinear_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_rectilinear_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_rectilinear_grid_output(self.0) }
    }
    fn get_graph_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_graph_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_graph_output(self.0) }
    }
    fn get_molecule_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_molecule_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_molecule_output(self.0) }
    }
    fn get_table_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_table_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_table_output(self.0) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_pass_input_type_algorithm_get_input(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_pass_input_type_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_pass_input_type_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_pass_input_type_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkPiecewiseFunctionAlgorithm for vtkPiecewiseFunctionAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_piecewise_function_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_piecewise_function_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_piecewise_function_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_piecewise_function_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_piecewise_function_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_piecewise_function_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_piecewise_function_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_piecewise_function_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_piecewise_function_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_piecewise_function_algorithm_set_output(self.0, d) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_piecewise_function_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_piecewise_function_algorithm_get_input(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_piecewise_function_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_piecewise_function_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_piecewise_function_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_piecewise_function_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkPiecewiseFunctionShiftScale for vtkPiecewiseFunctionShiftScale {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_piecewise_function_shift_scale_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_piecewise_function_shift_scale_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_piecewise_function_shift_scale_new_instance(self.0) }
    }
    fn set_position_shift(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_set_position_shift(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_piecewise_function_shift_scale_set_position_shift(self.0, _arg) }
    }
    fn set_position_scale(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_set_position_scale(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_piecewise_function_shift_scale_set_position_scale(self.0, _arg) }
    }
    fn set_value_shift(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_set_value_shift(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_piecewise_function_shift_scale_set_value_shift(self.0, _arg) }
    }
    fn set_value_scale(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_set_value_scale(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_piecewise_function_shift_scale_set_value_scale(self.0, _arg) }
    }
    fn get_position_shift(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_get_position_shift(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_piecewise_function_shift_scale_get_position_shift(self.0) }
    }
    fn get_position_scale(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_get_position_scale(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_piecewise_function_shift_scale_get_position_scale(self.0) }
    }
    fn get_value_shift(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_get_value_shift(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_piecewise_function_shift_scale_get_value_shift(self.0) }
    }
    fn get_value_scale(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_piecewise_function_shift_scale_get_value_scale(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_piecewise_function_shift_scale_get_value_scale(self.0) }
    }
}
impl VtkPointSetAlgorithm for vtkPointSetAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_set_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_set_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_set_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_set_algorithm_get_output(self.0) }
    }
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_get_poly_data_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_set_algorithm_get_poly_data_output(self.0) }
    }
    fn get_structured_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_get_structured_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_set_algorithm_get_structured_grid_output(self.0) }
    }
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_get_unstructured_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_set_algorithm_get_unstructured_grid_output(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_point_set_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_point_set_algorithm_add_input_data(self.0, p0) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_set_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_set_algorithm_get_input(self.0) }
    }
}
impl VtkPolyDataAlgorithm for vtkPolyDataAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_data_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_data_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_data_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_data_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_poly_data_algorithm_set_output(self.0, d) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_data_algorithm_get_input(self.0) }
    }
    fn get_poly_data_input(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_get_poly_data_input(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_data_algorithm_get_poly_data_input(self.0, port) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_poly_data_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_poly_data_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_poly_data_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkProgressObserver for vtkProgressObserver {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_progress_observer_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_progress_observer_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_progress_observer_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_progress_observer_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_progress_observer_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_progress_observer_new_instance(self.0) }
    }
    fn update_progress(&mut self, amount: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_progress_observer_update_progress(
                sself: *mut core::ffi::c_void,
                amount: core::ffi::c_double,
            );
        }
        unsafe { vtk_progress_observer_update_progress(self.0, amount) }
    }
    fn get_progress(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_progress_observer_get_progress(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_progress_observer_get_progress(self.0) }
    }
}
impl VtkReaderExecutive for vtkReaderExecutive {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_reader_executive_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_reader_executive_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_reader_executive_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_reader_executive_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_reader_executive_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_reader_executive_new_instance(self.0) }
    }
}
impl VtkRectilinearGridAlgorithm for vtkRectilinearGridAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_rectilinear_grid_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_rectilinear_grid_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_rectilinear_grid_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_rectilinear_grid_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_rectilinear_grid_algorithm_set_output(self.0, d) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_rectilinear_grid_algorithm_get_input(self.0) }
    }
    fn get_rectilinear_grid_input(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_get_rectilinear_grid_input(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_rectilinear_grid_algorithm_get_rectilinear_grid_input(self.0, port)
        }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_rectilinear_grid_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_rectilinear_grid_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_rectilinear_grid_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkSMPProgressObserver for vtkSMPProgressObserver {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_smp_progress_observer_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_smp_progress_observer_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_smp_progress_observer_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_smp_progress_observer_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_smp_progress_observer_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_smp_progress_observer_new_instance(self.0) }
    }
    fn update_progress(&mut self, progress: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_smp_progress_observer_update_progress(
                sself: *mut core::ffi::c_void,
                progress: core::ffi::c_double,
            );
        }
        unsafe { vtk_smp_progress_observer_update_progress(self.0, progress) }
    }
    fn get_local_observer(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_smp_progress_observer_get_local_observer(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_smp_progress_observer_get_local_observer(self.0) }
    }
}
impl VtkSelectionAlgorithm for vtkSelectionAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_selection_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_selection_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_selection_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_selection_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_selection_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_selection_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_selection_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_selection_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_selection_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_selection_algorithm_set_input_data(self.0, obj) }
    }
}
impl VtkSimpleScalarTree for vtkSimpleScalarTree {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_simple_scalar_tree_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_simple_scalar_tree_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_simple_scalar_tree_new_instance(self.0) }
    }
    fn set_branching_factor(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_set_branching_factor(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_simple_scalar_tree_set_branching_factor(self.0, _arg) }
    }
    fn get_branching_factor_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_branching_factor_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_simple_scalar_tree_get_branching_factor_min_value(self.0) }
    }
    fn get_branching_factor_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_branching_factor_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_simple_scalar_tree_get_branching_factor_max_value(self.0) }
    }
    fn get_branching_factor(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_branching_factor(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_simple_scalar_tree_get_branching_factor(self.0) }
    }
    fn get_level(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_level(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_simple_scalar_tree_get_level(self.0) }
    }
    fn set_max_level(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_set_max_level(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_simple_scalar_tree_set_max_level(self.0, _arg) }
    }
    fn get_max_level_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_max_level_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_simple_scalar_tree_get_max_level_min_value(self.0) }
    }
    fn get_max_level_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_max_level_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_simple_scalar_tree_get_max_level_max_value(self.0) }
    }
    fn get_max_level(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_max_level(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_simple_scalar_tree_get_max_level(self.0) }
    }
    fn build_tree(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_build_tree(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_simple_scalar_tree_build_tree(self.0) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_simple_scalar_tree_initialize(self.0) }
    }
    fn init_traversal(&mut self, scalarValue: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_init_traversal(
                sself: *mut core::ffi::c_void,
                scalarValue: core::ffi::c_double,
            );
        }
        unsafe { vtk_simple_scalar_tree_init_traversal(self.0, scalarValue) }
    }
    fn get_next_cell(
        &mut self,
        cellId: &mut core::ffi::c_longlong,
        ptIds: *mut core::ffi::c_void,
        cellScalars: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_next_cell(
                sself: *mut core::ffi::c_void,
                cellId: &mut core::ffi::c_longlong,
                ptIds: *mut core::ffi::c_void,
                cellScalars: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_simple_scalar_tree_get_next_cell(self.0, cellId, ptIds, cellScalars)
        }
    }
    fn get_number_of_cell_batches(
        &mut self,
        scalarValue: core::ffi::c_double,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_simple_scalar_tree_get_number_of_cell_batches(
                sself: *mut core::ffi::c_void,
                scalarValue: core::ffi::c_double,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_simple_scalar_tree_get_number_of_cell_batches(self.0, scalarValue) }
    }
}
impl VtkSpanSpace for vtkSpanSpace {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_span_space_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_span_space_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_span_space_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_span_space_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_span_space_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_span_space_new_instance(self.0) }
    }
    fn set_scalar_range(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_span_space_set_scalar_range(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
            );
        }
        unsafe { vtk_span_space_set_scalar_range(self.0, _arg1, _arg2) }
    }
    fn set_compute_scalar_range(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_span_space_set_compute_scalar_range(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_span_space_set_compute_scalar_range(self.0, _arg) }
    }
    fn get_compute_scalar_range(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_span_space_get_compute_scalar_range(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_span_space_get_compute_scalar_range(self.0) }
    }
    fn compute_scalar_range_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_span_space_compute_scalar_range_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_span_space_compute_scalar_range_on(self.0) }
    }
    fn compute_scalar_range_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_span_space_compute_scalar_range_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_span_space_compute_scalar_range_off(self.0) }
    }
    fn set_resolution(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_span_space_set_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_span_space_set_resolution(self.0, _arg) }
    }
    fn get_resolution_min_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_span_space_get_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_span_space_get_resolution_min_value(self.0) }
    }
    fn get_resolution_max_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_span_space_get_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_span_space_get_resolution_max_value(self.0) }
    }
    fn get_resolution(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_span_space_get_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_span_space_get_resolution(self.0) }
    }
    fn set_compute_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_span_space_set_compute_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_span_space_set_compute_resolution(self.0, _arg) }
    }
    fn get_compute_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_span_space_get_compute_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_span_space_get_compute_resolution(self.0) }
    }
    fn compute_resolution_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_span_space_compute_resolution_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_span_space_compute_resolution_on(self.0) }
    }
    fn compute_resolution_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_span_space_compute_resolution_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_span_space_compute_resolution_off(self.0) }
    }
    fn set_number_of_cells_per_bucket(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_span_space_set_number_of_cells_per_bucket(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_span_space_set_number_of_cells_per_bucket(self.0, _arg) }
    }
    fn get_number_of_cells_per_bucket_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_span_space_get_number_of_cells_per_bucket_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_span_space_get_number_of_cells_per_bucket_min_value(self.0) }
    }
    fn get_number_of_cells_per_bucket_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_span_space_get_number_of_cells_per_bucket_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_span_space_get_number_of_cells_per_bucket_max_value(self.0) }
    }
    fn get_number_of_cells_per_bucket(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_span_space_get_number_of_cells_per_bucket(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_span_space_get_number_of_cells_per_bucket(self.0) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_span_space_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_span_space_initialize(self.0) }
    }
    fn build_tree(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_span_space_build_tree(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_span_space_build_tree(self.0) }
    }
    fn init_traversal(&mut self, scalarValue: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_span_space_init_traversal(
                sself: *mut core::ffi::c_void,
                scalarValue: core::ffi::c_double,
            );
        }
        unsafe { vtk_span_space_init_traversal(self.0, scalarValue) }
    }
    fn get_next_cell(
        &mut self,
        cellId: &mut core::ffi::c_longlong,
        ptIds: *mut core::ffi::c_void,
        cellScalars: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_span_space_get_next_cell(
                sself: *mut core::ffi::c_void,
                cellId: &mut core::ffi::c_longlong,
                ptIds: *mut core::ffi::c_void,
                cellScalars: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_span_space_get_next_cell(self.0, cellId, ptIds, cellScalars) }
    }
    fn get_number_of_cell_batches(
        &mut self,
        scalarValue: core::ffi::c_double,
    ) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_span_space_get_number_of_cell_batches(
                sself: *mut core::ffi::c_void,
                scalarValue: core::ffi::c_double,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_span_space_get_number_of_cell_batches(self.0, scalarValue) }
    }
    fn set_batch_size(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_span_space_set_batch_size(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_span_space_set_batch_size(self.0, _arg) }
    }
    fn get_batch_size_min_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_span_space_get_batch_size_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_span_space_get_batch_size_min_value(self.0) }
    }
    fn get_batch_size_max_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_span_space_get_batch_size_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_span_space_get_batch_size_max_value(self.0) }
    }
    fn get_batch_size(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_span_space_get_batch_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_span_space_get_batch_size(self.0) }
    }
}
impl VtkSphereTree for vtkSphereTree {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sphere_tree_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sphere_tree_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sphere_tree_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sphere_tree_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sphere_tree_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sphere_tree_new_instance(self.0) }
    }
    fn set_data_set(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_sphere_tree_set_data_set(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_sphere_tree_set_data_set(self.0, p0) }
    }
    fn get_data_set(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_data_set(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sphere_tree_get_data_set(self.0) }
    }
    fn build(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_sphere_tree_build(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_sphere_tree_build(self.0) }
    }
    fn set_build_hierarchy(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_sphere_tree_set_build_hierarchy(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_sphere_tree_set_build_hierarchy(self.0, _arg) }
    }
    fn get_build_hierarchy(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_build_hierarchy(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_sphere_tree_get_build_hierarchy(self.0) }
    }
    fn build_hierarchy_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_sphere_tree_build_hierarchy_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_sphere_tree_build_hierarchy_on(self.0) }
    }
    fn build_hierarchy_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_sphere_tree_build_hierarchy_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_sphere_tree_build_hierarchy_off(self.0) }
    }
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_sphere_tree_set_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_sphere_tree_set_resolution(self.0, _arg) }
    }
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_tree_get_resolution_min_value(self.0) }
    }
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_tree_get_resolution_max_value(self.0) }
    }
    fn get_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_tree_get_resolution(self.0) }
    }
    fn set_max_level(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_sphere_tree_set_max_level(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_sphere_tree_set_max_level(self.0, _arg) }
    }
    fn get_max_level_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_max_level_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_tree_get_max_level_min_value(self.0) }
    }
    fn get_max_level_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_max_level_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_tree_get_max_level_max_value(self.0) }
    }
    fn get_max_level(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_max_level(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_tree_get_max_level(self.0) }
    }
    fn get_number_of_levels(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_tree_get_number_of_levels(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_tree_get_number_of_levels(self.0) }
    }
}
impl VtkStreamingDemandDrivenPipeline for vtkStreamingDemandDrivenPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_new_instance(self.0) }
    }
    fn update(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_update(self.0) }
    }
    fn update_whole_extent(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update_whole_extent(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_update_whole_extent(self.0) }
    }
    fn propagate_update_extent(
        &mut self,
        outputPort: core::ffi::c_int,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_propagate_update_extent(
                sself: *mut core::ffi::c_void,
                outputPort: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_propagate_update_extent(
                self.0,
                outputPort,
            )
        }
    }
    fn propagate_time(&mut self, outputPort: core::ffi::c_int) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_propagate_time(
                sself: *mut core::ffi::c_void,
                outputPort: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_propagate_time(self.0, outputPort)
        }
    }
    fn update_time_dependent_information(
        &mut self,
        outputPort: core::ffi::c_int,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update_time_dependent_information(
                sself: *mut core::ffi::c_void,
                outputPort: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_update_time_dependent_information(
                self.0,
                outputPort,
            )
        }
    }
    fn set_request_exact_extent(
        &mut self,
        port: core::ffi::c_int,
        flag: core::ffi::c_int,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_set_request_exact_extent(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
                flag: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_set_request_exact_extent(
                self.0,
                port,
                flag,
            )
        }
    }
    fn get_request_exact_extent(&mut self, port: core::ffi::c_int) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_get_request_exact_extent(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_get_request_exact_extent(self.0, port)
        }
    }
    fn request_update_extent(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_request_update_extent(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_request_update_extent(self.0) }
    }
    fn request_update_time(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_request_update_time(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_request_update_time(self.0) }
    }
    fn request_time_dependent_information(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_request_time_dependent_information(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_request_time_dependent_information(
                self.0,
            )
        }
    }
    fn continue_executing(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_continue_executing(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_continue_executing(self.0) }
    }
    fn update_extent_initialized(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update_extent_initialized(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_update_extent_initialized(self.0) }
    }
    fn update_extent(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update_extent(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_update_extent(self.0) }
    }
    fn update_piece_number(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update_piece_number(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_update_piece_number(self.0) }
    }
    fn update_number_of_pieces(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update_number_of_pieces(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_update_number_of_pieces(self.0) }
    }
    fn update_number_of_ghost_levels(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update_number_of_ghost_levels(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_update_number_of_ghost_levels(self.0)
        }
    }
    fn combined_update_extent(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_combined_update_extent(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_combined_update_extent(self.0) }
    }
    fn whole_extent(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_whole_extent(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_whole_extent(self.0) }
    }
    fn unrestricted_update_extent(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_unrestricted_update_extent(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_unrestricted_update_extent(self.0)
        }
    }
    fn exact_extent(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_exact_extent(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_exact_extent(self.0) }
    }
    fn time_steps(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_time_steps(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_time_steps(self.0) }
    }
    fn time_range(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_time_range(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_time_range(self.0) }
    }
    fn update_time_step(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_update_time_step(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_update_time_step(self.0) }
    }
    fn time_dependent_information(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_time_dependent_information(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_time_dependent_information(self.0)
        }
    }
    fn bounds(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_bounds(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_bounds(self.0) }
    }
    fn get_update_piece(&mut self, p0: *mut core::ffi::c_void) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_get_update_piece(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_streaming_demand_driven_pipeline_get_update_piece(self.0, p0) }
    }
    fn get_update_number_of_pieces(
        &mut self,
        p0: *mut core::ffi::c_void,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_get_update_number_of_pieces(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_get_update_number_of_pieces(self.0, p0)
        }
    }
    fn get_update_ghost_level(
        &mut self,
        p0: *mut core::ffi::c_void,
    ) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_streaming_demand_driven_pipeline_get_update_ghost_level(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_streaming_demand_driven_pipeline_get_update_ghost_level(self.0, p0)
        }
    }
}
impl VtkStructuredGridAlgorithm for vtkStructuredGridAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_structured_grid_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_structured_grid_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_structured_grid_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_structured_grid_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_structured_grid_algorithm_set_output(self.0, d) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_structured_grid_algorithm_get_input(self.0) }
    }
    fn get_structured_grid_input(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_get_structured_grid_input(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_structured_grid_algorithm_get_structured_grid_input(self.0, port) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_structured_grid_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_structured_grid_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_structured_grid_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkTableAlgorithm for vtkTableAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_table_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_table_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_table_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_table_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_table_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_table_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_table_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_table_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_table_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_table_algorithm_set_input_data(self.0, obj) }
    }
}
impl VtkThreadedCompositeDataPipeline for vtkThreadedCompositeDataPipeline {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_threaded_composite_data_pipeline_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_threaded_composite_data_pipeline_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_threaded_composite_data_pipeline_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_threaded_composite_data_pipeline_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_threaded_composite_data_pipeline_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_threaded_composite_data_pipeline_new_instance(self.0) }
    }
}
impl VtkTreeAlgorithm for vtkTreeAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_tree_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_tree_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_tree_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_tree_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_tree_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_tree_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_tree_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_tree_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_tree_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_tree_algorithm_set_input_data(self.0, obj) }
    }
}
impl VtkTrivialConsumer for vtkTrivialConsumer {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_trivial_consumer_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_trivial_consumer_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_trivial_consumer_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_trivial_consumer_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_trivial_consumer_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_trivial_consumer_new_instance(self.0) }
    }
}
impl VtkTrivialProducer for vtkTrivialProducer {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_trivial_producer_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_trivial_producer_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_trivial_producer_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_trivial_producer_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_trivial_producer_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_trivial_producer_new_instance(self.0) }
    }
    fn set_output(&mut self, output: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_trivial_producer_set_output(
                sself: *mut core::ffi::c_void,
                output: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_trivial_producer_set_output(self.0, output) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_trivial_producer_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_trivial_producer_get_m_time(self.0) }
    }
    fn set_whole_extent(
        &mut self,
        _arg1: core::ffi::c_int,
        _arg2: core::ffi::c_int,
        _arg3: core::ffi::c_int,
        _arg4: core::ffi::c_int,
        _arg5: core::ffi::c_int,
        _arg6: core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_trivial_producer_set_whole_extent(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_int,
                _arg2: core::ffi::c_int,
                _arg3: core::ffi::c_int,
                _arg4: core::ffi::c_int,
                _arg5: core::ffi::c_int,
                _arg6: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_trivial_producer_set_whole_extent(
                self.0,
                _arg1,
                _arg2,
                _arg3,
                _arg4,
                _arg5,
                _arg6,
            )
        }
    }
    fn fill_output_data_information(
        &mut self,
        output: *mut core::ffi::c_void,
        outInfo: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_trivial_producer_fill_output_data_information(
                sself: *mut core::ffi::c_void,
                output: *mut core::ffi::c_void,
                outInfo: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtk_trivial_producer_fill_output_data_information(self.0, output, outInfo)
        }
    }
}
impl VtkUndirectedGraphAlgorithm for vtkUndirectedGraphAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_undirected_graph_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_undirected_graph_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_undirected_graph_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_undirected_graph_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_undirected_graph_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_undirected_graph_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_undirected_graph_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_undirected_graph_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, obj: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_undirected_graph_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                obj: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_undirected_graph_algorithm_set_input_data(self.0, obj) }
    }
}
impl VtkUniformGridAMRAlgorithm for vtkUniformGridAMRAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_grid_amr_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_grid_amr_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_grid_amr_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_grid_amr_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_grid_amr_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_grid_amr_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_grid_amr_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_grid_amr_algorithm_get_output(self.0) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_uniform_grid_amr_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_uniform_grid_amr_algorithm_set_input_data(self.0, p0) }
    }
}
impl VtkUniformGridPartitioner for vtkUniformGridPartitioner {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_grid_partitioner_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_grid_partitioner_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_grid_partitioner_new_instance(self.0) }
    }
    fn get_number_of_partitions(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_get_number_of_partitions(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_uniform_grid_partitioner_get_number_of_partitions(self.0) }
    }
    fn set_number_of_partitions(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_set_number_of_partitions(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_uniform_grid_partitioner_set_number_of_partitions(self.0, _arg) }
    }
    fn get_number_of_ghost_layers(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_get_number_of_ghost_layers(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_uniform_grid_partitioner_get_number_of_ghost_layers(self.0) }
    }
    fn set_number_of_ghost_layers(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_set_number_of_ghost_layers(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_uniform_grid_partitioner_set_number_of_ghost_layers(self.0, _arg) }
    }
    fn get_duplicate_nodes(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_get_duplicate_nodes(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_uniform_grid_partitioner_get_duplicate_nodes(self.0) }
    }
    fn set_duplicate_nodes(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_set_duplicate_nodes(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_uniform_grid_partitioner_set_duplicate_nodes(self.0, _arg) }
    }
    fn duplicate_nodes_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_duplicate_nodes_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_uniform_grid_partitioner_duplicate_nodes_on(self.0) }
    }
    fn duplicate_nodes_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_uniform_grid_partitioner_duplicate_nodes_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_uniform_grid_partitioner_duplicate_nodes_off(self.0) }
    }
}
impl VtkUnstructuredGridAlgorithm for vtkUnstructuredGridAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unstructured_grid_algorithm_set_output(self.0, d) }
    }
    fn get_input(&mut self, port: core::ffi::c_int) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_get_input(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_algorithm_get_input(self.0, port) }
    }
    fn get_unstructured_grid_input(
        &mut self,
        port: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_get_unstructured_grid_input(
                sself: *mut core::ffi::c_void,
                port: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_unstructured_grid_algorithm_get_unstructured_grid_input(self.0, port)
        }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unstructured_grid_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_unstructured_grid_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unstructured_grid_algorithm_add_input_data(self.0, p0) }
    }
}
impl VtkUnstructuredGridBaseAlgorithm for vtkUnstructuredGridBaseAlgorithm {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_base_algorithm_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_base_algorithm_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_base_algorithm_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_base_algorithm_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_base_algorithm_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_base_algorithm_new_instance(self.0) }
    }
    fn get_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_unstructured_grid_base_algorithm_get_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_unstructured_grid_base_algorithm_get_output(self.0) }
    }
    fn set_output(&mut self, d: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_unstructured_grid_base_algorithm_set_output(
                sself: *mut core::ffi::c_void,
                d: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unstructured_grid_base_algorithm_set_output(self.0, d) }
    }
    fn set_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_unstructured_grid_base_algorithm_set_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unstructured_grid_base_algorithm_set_input_data(self.0, p0) }
    }
    fn add_input_data(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_unstructured_grid_base_algorithm_add_input_data(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_unstructured_grid_base_algorithm_add_input_data(self.0, p0) }
    }
}
/// Superclass for all sources, filters, and sinks in VTK.
///
///
/// vtkAlgorithm is the superclass for all sources, filters, and sinks
/// in VTK.  It defines a generalized interface for executing data
/// processing algorithms.  Pipeline connections are associated with
/// input and output ports that are independent of the type of data
/// passing through the connections.
///
/// Instances may be used independently or within pipelines with a
/// variety of architectures and update mechanisms.  Pipelines are
/// controlled by instances of vtkExecutive.  Every vtkAlgorithm
/// instance has an associated vtkExecutive when it is used in a
/// pipeline.  The executive is responsible for data flow.
#[allow(non_camel_case_types)]
pub struct vtkAlgorithm(*mut core::ffi::c_void);
impl vtkAlgorithm {
    /// Creates a new [vtkAlgorithm] via `vtkAlgorithm::New()`
    #[doc(alias = "vtkAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkAlgorithm_new() })
    }
}
impl std::default::Default for vtkAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAlgorithm_create_drop() {
    let obj = vtkAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Proxy object to connect input/output ports.
///
///
/// vtkAlgorithmOutput is a proxy object returned by the GetOutputPort
/// method of vtkAlgorithm.  It may be passed to the
/// SetInputConnection, AddInputConnection, or RemoveInputConnection
/// methods of another vtkAlgorithm to establish a connection between
/// an output and input port.  The connection is not stored in the
/// proxy object: it is simply a convenience for creating or removing
/// connections.
#[allow(non_camel_case_types)]
pub struct vtkAlgorithmOutput(*mut core::ffi::c_void);
impl vtkAlgorithmOutput {
    /// Creates a new [vtkAlgorithmOutput] via `vtkAlgorithmOutput::New()`
    #[doc(alias = "vtkAlgorithmOutput")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAlgorithmOutput_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkAlgorithmOutput_new() })
    }
}
impl std::default::Default for vtkAlgorithmOutput {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAlgorithmOutput {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAlgorithmOutput_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAlgorithmOutput_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAlgorithmOutput_create_drop() {
    let obj = vtkAlgorithmOutput::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only vtkAnnotationLayers as output
///
///
///
/// vtkAnnotationLayersAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be vtkAnnotationLayers. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkAnnotationLayersAlgorithm(*mut core::ffi::c_void);
impl vtkAnnotationLayersAlgorithm {
    /// Creates a new [vtkAnnotationLayersAlgorithm] via `vtkAnnotationLayersAlgorithm::New()`
    #[doc(alias = "vtkAnnotationLayersAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAnnotationLayersAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkAnnotationLayersAlgorithm_new() })
    }
}
impl std::default::Default for vtkAnnotationLayersAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAnnotationLayersAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAnnotationLayersAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAnnotationLayersAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAnnotationLayersAlgorithm_create_drop() {
    let obj = vtkAnnotationLayersAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce
///
/// vtkArrayDatas as output
///
///
/// vtkArrayDataAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be vtkArrayData. If that
/// isn't the case then please override this method in your subclass.
///
/// @par Thanks:
/// Developed by Timothy M. Shead (tshead@sandia.gov) at Sandia National Laboratories.
#[allow(non_camel_case_types)]
pub struct vtkArrayDataAlgorithm(*mut core::ffi::c_void);
impl vtkArrayDataAlgorithm {
    /// Creates a new [vtkArrayDataAlgorithm] via `vtkArrayDataAlgorithm::New()`
    #[doc(alias = "vtkArrayDataAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkArrayDataAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkArrayDataAlgorithm_new() })
    }
}
impl std::default::Default for vtkArrayDataAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkArrayDataAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkArrayDataAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkArrayDataAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkArrayDataAlgorithm_create_drop() {
    let obj = vtkArrayDataAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
/// vtkCachedStreamingDemandDrivenPipeline
#[allow(non_camel_case_types)]
pub struct vtkCachedStreamingDemandDrivenPipeline(*mut core::ffi::c_void);
impl vtkCachedStreamingDemandDrivenPipeline {
    /// Creates a new [vtkCachedStreamingDemandDrivenPipeline] via `vtkCachedStreamingDemandDrivenPipeline::New()`
    #[doc(alias = "vtkCachedStreamingDemandDrivenPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCachedStreamingDemandDrivenPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCachedStreamingDemandDrivenPipeline_new() })
    }
}
impl std::default::Default for vtkCachedStreamingDemandDrivenPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCachedStreamingDemandDrivenPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCachedStreamingDemandDrivenPipeline_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkCachedStreamingDemandDrivenPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCachedStreamingDemandDrivenPipeline_create_drop() {
    let obj = vtkCachedStreamingDemandDrivenPipeline::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// works around type-checking limitations
///
///
/// vtkCastToConcrete is a filter that works around type-checking limitations
/// in the filter classes. Some filters generate abstract types on output,
/// and cannot be connected to the input of filters requiring a concrete
/// input type. For example, vtkElevationFilter generates vtkDataSet for output,
/// and cannot be connected to vtkDecimate, because vtkDecimate requires
/// vtkPolyData as input. This is true even though (in this example) the input
/// to vtkElevationFilter is of type vtkPolyData, and you know the output of
/// vtkElevationFilter is the same type as its input.
///
/// vtkCastToConcrete performs run-time checking to ensure that output type
/// is of the right type. An error message will result if you try to cast
/// an input type improperly. Otherwise, the filter performs the appropriate
/// cast and returns the data.
///
/// @warning
/// You must specify the input before you can get the output. Otherwise an
/// error results.
///
/// @sa
/// vtkDataSetAlgorithm vtkPointSetToPointSetFilter
#[allow(non_camel_case_types)]
pub struct vtkCastToConcrete(*mut core::ffi::c_void);
impl vtkCastToConcrete {
    /// Creates a new [vtkCastToConcrete] via `vtkCastToConcrete::New()`
    #[doc(alias = "vtkCastToConcrete")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCastToConcrete_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCastToConcrete_new() })
    }
}
impl std::default::Default for vtkCastToConcrete {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCastToConcrete {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCastToConcrete_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCastToConcrete_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCastToConcrete_create_drop() {
    let obj = vtkCastToConcrete::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Executive supporting composite datasets.
///
///
/// vtkCompositeDataPipeline is an executive that supports the processing of
/// composite dataset. It supports algorithms that are aware of composite
/// dataset as well as those that are not. Type checking is performed at run
/// time. Algorithms that are not composite dataset-aware have to support
/// all dataset types contained in the composite dataset. The pipeline
/// execution can be summarized as follows:
///
/// * REQUEST_INFORMATION: The producers have to provide information about
/// the contents of the composite dataset in this pass.
/// Sources that can produce more than one piece (note that a piece is
/// different than a block; each piece consistes of 0 or more blocks) should
/// set CAN_HANDLE_PIECE_REQUEST.
///
/// * REQUEST_UPDATE_EXTENT: This pass is identical to the one implemented
/// in vtkStreamingDemandDrivenPipeline
///
/// * REQUEST_DATA: This is where the algorithms execute. If the
/// vtkCompositeDataPipeline is assigned to a simple filter,
/// it will invoke the  vtkStreamingDemandDrivenPipeline passes in a loop,
/// passing a different block each time and will collect the results in a
/// composite dataset.
/// @sa
/// vtkCompositeDataSet
#[allow(non_camel_case_types)]
pub struct vtkCompositeDataPipeline(*mut core::ffi::c_void);
impl vtkCompositeDataPipeline {
    /// Creates a new [vtkCompositeDataPipeline] via `vtkCompositeDataPipeline::New()`
    #[doc(alias = "vtkCompositeDataPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCompositeDataPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCompositeDataPipeline_new() })
    }
}
impl std::default::Default for vtkCompositeDataPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCompositeDataPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCompositeDataPipeline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCompositeDataPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCompositeDataPipeline_create_drop() {
    let obj = vtkCompositeDataPipeline::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only vtkCompositeDataSet as output
///
///
/// Algorithms that take any type of data object (including composite dataset)
/// and produce a vtkCompositeDataSet in the output can subclass from this
/// class.
#[allow(non_camel_case_types)]
pub struct vtkCompositeDataSetAlgorithm(*mut core::ffi::c_void);
impl vtkCompositeDataSetAlgorithm {
    /// Creates a new [vtkCompositeDataSetAlgorithm] via `vtkCompositeDataSetAlgorithm::New()`
    #[doc(alias = "vtkCompositeDataSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCompositeDataSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCompositeDataSetAlgorithm_new() })
    }
}
impl std::default::Default for vtkCompositeDataSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCompositeDataSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCompositeDataSetAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCompositeDataSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCompositeDataSetAlgorithm_create_drop() {
    let obj = vtkCompositeDataSetAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only data object as output
///
///
///
/// vtkDataObjectAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be DataObject. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkDataObjectAlgorithm(*mut core::ffi::c_void);
impl vtkDataObjectAlgorithm {
    /// Creates a new [vtkDataObjectAlgorithm] via `vtkDataObjectAlgorithm::New()`
    #[doc(alias = "vtkDataObjectAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataObjectAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDataObjectAlgorithm_new() })
    }
}
impl std::default::Default for vtkDataObjectAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataObjectAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataObjectAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataObjectAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataObjectAlgorithm_create_drop() {
    let obj = vtkDataObjectAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce output of the same type as input
///
///
/// vtkDataSetAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be DataSet. If that isn't
/// the case then please override this method in your subclass. This class
/// breaks out the downstream requests into separate functions such as
/// RequestDataObject RequestData and RequestInformation. The default
/// implementation of RequestDataObject will create an output data of the
/// same type as the input.
#[allow(non_camel_case_types)]
pub struct vtkDataSetAlgorithm(*mut core::ffi::c_void);
impl vtkDataSetAlgorithm {
    /// Creates a new [vtkDataSetAlgorithm] via `vtkDataSetAlgorithm::New()`
    #[doc(alias = "vtkDataSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDataSetAlgorithm_new() })
    }
}
impl std::default::Default for vtkDataSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataSetAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataSetAlgorithm_create_drop() {
    let obj = vtkDataSetAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Executive supporting on-demand execution.
///
///
/// vtkDemandDrivenPipeline is an executive that will execute an
/// algorithm only when its outputs are out-of-date with respect to its
/// inputs.
#[allow(non_camel_case_types)]
pub struct vtkDemandDrivenPipeline(*mut core::ffi::c_void);
impl vtkDemandDrivenPipeline {
    /// Creates a new [vtkDemandDrivenPipeline] via `vtkDemandDrivenPipeline::New()`
    #[doc(alias = "vtkDemandDrivenPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDemandDrivenPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDemandDrivenPipeline_new() })
    }
}
impl std::default::Default for vtkDemandDrivenPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDemandDrivenPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDemandDrivenPipeline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDemandDrivenPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDemandDrivenPipeline_create_drop() {
    let obj = vtkDemandDrivenPipeline::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only directed graph as output
///
///
///
/// vtkDirectedGraphAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline edgehitecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Graph. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
///
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkDirectedGraphAlgorithm(*mut core::ffi::c_void);
impl vtkDirectedGraphAlgorithm {
    /// Creates a new [vtkDirectedGraphAlgorithm] via `vtkDirectedGraphAlgorithm::New()`
    #[doc(alias = "vtkDirectedGraphAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDirectedGraphAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDirectedGraphAlgorithm_new() })
    }
}
impl std::default::Default for vtkDirectedGraphAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDirectedGraphAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDirectedGraphAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDirectedGraphAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDirectedGraphAlgorithm_create_drop() {
    let obj = vtkDirectedGraphAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// source that manages dataset ensembles
///
///
/// vtkEnsembleSource manages a collection of data sources in order to
/// represent a dataset ensemble. It has the ability to provide meta-data
/// about the ensemble in the form of a table, using the META_DATA key
/// as well as accept a pipeline request using the UPDATE_MEMBER key.
/// Note that it is expected that all ensemble members produce data of the
/// same type.
#[allow(non_camel_case_types)]
pub struct vtkEnsembleSource(*mut core::ffi::c_void);
impl vtkEnsembleSource {
    /// Creates a new [vtkEnsembleSource] via `vtkEnsembleSource::New()`
    #[doc(alias = "vtkEnsembleSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEnsembleSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkEnsembleSource_new() })
    }
}
impl std::default::Default for vtkEnsembleSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEnsembleSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEnsembleSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEnsembleSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEnsembleSource_create_drop() {
    let obj = vtkEnsembleSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only
///
/// explicit structured grid as output.
#[allow(non_camel_case_types)]
pub struct vtkExplicitStructuredGridAlgorithm(*mut core::ffi::c_void);
impl vtkExplicitStructuredGridAlgorithm {
    /// Creates a new [vtkExplicitStructuredGridAlgorithm] via `vtkExplicitStructuredGridAlgorithm::New()`
    #[doc(alias = "vtkExplicitStructuredGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExplicitStructuredGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkExplicitStructuredGridAlgorithm_new() })
    }
}
impl std::default::Default for vtkExplicitStructuredGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExplicitStructuredGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExplicitStructuredGridAlgorithm_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkExplicitStructuredGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExplicitStructuredGridAlgorithm_create_drop() {
    let obj = vtkExplicitStructuredGridAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// This method partitions a global extent to N partitions where N is a user
///
/// supplied parameter.
#[allow(non_camel_case_types)]
pub struct vtkExtentRCBPartitioner(*mut core::ffi::c_void);
impl vtkExtentRCBPartitioner {
    /// Creates a new [vtkExtentRCBPartitioner] via `vtkExtentRCBPartitioner::New()`
    #[doc(alias = "vtkExtentRCBPartitioner")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExtentRCBPartitioner_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkExtentRCBPartitioner_new() })
    }
}
impl std::default::Default for vtkExtentRCBPartitioner {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExtentRCBPartitioner {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExtentRCBPartitioner_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExtentRCBPartitioner_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExtentRCBPartitioner_create_drop() {
    let obj = vtkExtentRCBPartitioner::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Split an extent across other extents.
///
///
/// vtkExtentSplitter splits each input extent into non-overlapping
/// sub-extents that are completely contained within other "source
/// extents".  A source extent corresponds to some resource providing
/// an extent.  Each source extent has an integer identifier, integer
/// priority, and an extent.  The input extents are split into
/// sub-extents according to priority, availability, and amount of
/// overlap of the source extents.  This can be used by parallel data
/// readers to read as few piece files as possible.
#[allow(non_camel_case_types)]
pub struct vtkExtentSplitter(*mut core::ffi::c_void);
impl vtkExtentSplitter {
    /// Creates a new [vtkExtentSplitter] via `vtkExtentSplitter::New()`
    #[doc(alias = "vtkExtentSplitter")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExtentSplitter_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkExtentSplitter_new() })
    }
}
impl std::default::Default for vtkExtentSplitter {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExtentSplitter {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExtentSplitter_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExtentSplitter_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExtentSplitter_create_drop() {
    let obj = vtkExtentSplitter::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generates a structured extent from unstructured.
///
///
///
/// vtkExtentTranslator generates a structured extent from an unstructured
/// extent.  It uses a recursive scheme that splits the largest axis.  A hard
/// coded extent can be used for a starting point.
#[allow(non_camel_case_types)]
pub struct vtkExtentTranslator(*mut core::ffi::c_void);
impl vtkExtentTranslator {
    /// Creates a new [vtkExtentTranslator] via `vtkExtentTranslator::New()`
    #[doc(alias = "vtkExtentTranslator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExtentTranslator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkExtentTranslator_new() })
    }
}
impl std::default::Default for vtkExtentTranslator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExtentTranslator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExtentTranslator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExtentTranslator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExtentTranslator_create_drop() {
    let obj = vtkExtentTranslator::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only graph as output
///
///
///
/// vtkGraphAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Graph. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkGraphAlgorithm(*mut core::ffi::c_void);
impl vtkGraphAlgorithm {
    /// Creates a new [vtkGraphAlgorithm] via `vtkGraphAlgorithm::New()`
    #[doc(alias = "vtkGraphAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGraphAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkGraphAlgorithm_new() })
    }
}
impl std::default::Default for vtkGraphAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGraphAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGraphAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGraphAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGraphAlgorithm_create_drop() {
    let obj = vtkGraphAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// superclass for algorithms that
///
/// produce vtkHierarchicalBoxDataSet as output.
///
/// Algorithms that take any type of data object (including composite dataset)
/// and produce a vtkHierarchicalBoxDataSet in the output can subclass from this
/// class.
#[allow(non_camel_case_types)]
pub struct vtkHierarchicalBoxDataSetAlgorithm(*mut core::ffi::c_void);
impl vtkHierarchicalBoxDataSetAlgorithm {
    /// Creates a new [vtkHierarchicalBoxDataSetAlgorithm] via `vtkHierarchicalBoxDataSetAlgorithm::New()`
    #[doc(alias = "vtkHierarchicalBoxDataSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHierarchicalBoxDataSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkHierarchicalBoxDataSetAlgorithm_new() })
    }
}
impl std::default::Default for vtkHierarchicalBoxDataSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHierarchicalBoxDataSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHierarchicalBoxDataSetAlgorithm_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHierarchicalBoxDataSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHierarchicalBoxDataSetAlgorithm_create_drop() {
    let obj = vtkHierarchicalBoxDataSetAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
/// a structured grid instance.
///
///
/// A concrete instance of vtkStructuredGridAlgorithm which provides
/// functionality for converting instances of vtkImageData to vtkStructuredGrid.
#[allow(non_camel_case_types)]
pub struct vtkImageToStructuredGrid(*mut core::ffi::c_void);
impl vtkImageToStructuredGrid {
    /// Creates a new [vtkImageToStructuredGrid] via `vtkImageToStructuredGrid::New()`
    #[doc(alias = "vtkImageToStructuredGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImageToStructuredGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkImageToStructuredGrid_new() })
    }
}
impl std::default::Default for vtkImageToStructuredGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImageToStructuredGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImageToStructuredGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImageToStructuredGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImageToStructuredGrid_create_drop() {
    let obj = vtkImageToStructuredGrid::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Attaches image pipeline to VTK.
///
///
/// vtkImageToStructuredPoints changes an image cache format to
/// a structured points dataset.  It takes an Input plus an optional
/// VectorInput. The VectorInput converts the RGB scalar components
/// of the VectorInput to vector pointdata attributes. This filter
/// will try to reference count the data but in some cases it must
/// make a copy.
#[allow(non_camel_case_types)]
pub struct vtkImageToStructuredPoints(*mut core::ffi::c_void);
impl vtkImageToStructuredPoints {
    /// Creates a new [vtkImageToStructuredPoints] via `vtkImageToStructuredPoints::New()`
    #[doc(alias = "vtkImageToStructuredPoints")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImageToStructuredPoints_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkImageToStructuredPoints_new() })
    }
}
impl std::default::Default for vtkImageToStructuredPoints {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImageToStructuredPoints {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImageToStructuredPoints_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImageToStructuredPoints_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImageToStructuredPoints_create_drop() {
    let obj = vtkImageToStructuredPoints::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that operate on
///
/// vtkMolecules
///
///
///
/// vtkMoleculeAlgorithm is a convenience class to make writing algorithms
/// easier. There are some assumptions and defaults made by this class you
/// should be aware of. This class defaults such that your filter will have
/// one input port and one output port. If that is not the case simply change
/// it with SetNumberOfInputPorts etc. See this class constructor for the
/// default. This class also provides a FillInputPortInfo method that by
/// default says that all inputs will be vtkMolecules. If that isn't the case
/// then please override this method in your subclass. You should implement
/// the subclass's algorithm into RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkMoleculeAlgorithm(*mut core::ffi::c_void);
impl vtkMoleculeAlgorithm {
    /// Creates a new [vtkMoleculeAlgorithm] via `vtkMoleculeAlgorithm::New()`
    #[doc(alias = "vtkMoleculeAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMoleculeAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMoleculeAlgorithm_new() })
    }
}
impl std::default::Default for vtkMoleculeAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMoleculeAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMoleculeAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMoleculeAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMoleculeAlgorithm_create_drop() {
    let obj = vtkMoleculeAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only vtkMultiBlockDataSet as output
///
///
/// Algorithms that take any type of data object (including composite dataset)
/// and produce a vtkMultiBlockDataSet in the output can subclass from this
/// class.
#[allow(non_camel_case_types)]
pub struct vtkMultiBlockDataSetAlgorithm(*mut core::ffi::c_void);
impl vtkMultiBlockDataSetAlgorithm {
    /// Creates a new [vtkMultiBlockDataSetAlgorithm] via `vtkMultiBlockDataSetAlgorithm::New()`
    #[doc(alias = "vtkMultiBlockDataSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMultiBlockDataSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMultiBlockDataSetAlgorithm_new() })
    }
}
impl std::default::Default for vtkMultiBlockDataSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMultiBlockDataSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMultiBlockDataSetAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMultiBlockDataSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMultiBlockDataSetAlgorithm_create_drop() {
    let obj = vtkMultiBlockDataSetAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that would like to make multiple time requests
///
///
/// This class can be inherited by any algorithm that wishes to make multiple
/// time requests upstream.
///
/// A subclass should override `RequestUpdateExtent` and use
/// `vtkMultiTimeStepAlgorithm::UPDATE_TIME_STEPS` key to indicate which
/// timesteps are to be requested. This class will then take care of executing
/// the upstream pipeline to obtain the requested timesteps.
///
/// Subclasses can then override `Execute` which is provided a vector of input
/// data objects corresponding to the requested timesteps.
///
/// In VTK 9.1 and earlier, subclasses overrode `RequestData` instead of
/// `Execute`. RequestData was passed a `vtkMultiBlockDataSet` with blocks corresponding
/// to the input timesteps. However, with addition of vtkPartitionedDataSet and
/// vtkPartitionedDataSetCollection in VTK 9.2, it is not possible to package all
/// input data types into a multiblock dataset. Hence, the method is deprecated
/// and only used when `Execute` is not overridden.
#[allow(non_camel_case_types)]
pub struct vtkMultiTimeStepAlgorithm(*mut core::ffi::c_void);
impl vtkMultiTimeStepAlgorithm {
    /// Creates a new [vtkMultiTimeStepAlgorithm] via `vtkMultiTimeStepAlgorithm::New()`
    #[doc(alias = "vtkMultiTimeStepAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMultiTimeStepAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMultiTimeStepAlgorithm_new() })
    }
}
impl std::default::Default for vtkMultiTimeStepAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMultiTimeStepAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMultiTimeStepAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMultiTimeStepAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMultiTimeStepAlgorithm_create_drop() {
    let obj = vtkMultiTimeStepAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
/// produce vtkNonOverlappingAMR as output.
#[allow(non_camel_case_types)]
pub struct vtkNonOverlappingAMRAlgorithm(*mut core::ffi::c_void);
impl vtkNonOverlappingAMRAlgorithm {
    /// Creates a new [vtkNonOverlappingAMRAlgorithm] via `vtkNonOverlappingAMRAlgorithm::New()`
    #[doc(alias = "vtkNonOverlappingAMRAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkNonOverlappingAMRAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkNonOverlappingAMRAlgorithm_new() })
    }
}
impl std::default::Default for vtkNonOverlappingAMRAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkNonOverlappingAMRAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkNonOverlappingAMRAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkNonOverlappingAMRAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkNonOverlappingAMRAlgorithm_create_drop() {
    let obj = vtkNonOverlappingAMRAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// A base class for all algorithms that take as input vtkOverlappingAMR and
///
/// produce vtkOverlappingAMR.
#[allow(non_camel_case_types)]
pub struct vtkOverlappingAMRAlgorithm(*mut core::ffi::c_void);
impl vtkOverlappingAMRAlgorithm {
    /// Creates a new [vtkOverlappingAMRAlgorithm] via `vtkOverlappingAMRAlgorithm::New()`
    #[doc(alias = "vtkOverlappingAMRAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOverlappingAMRAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkOverlappingAMRAlgorithm_new() })
    }
}
impl std::default::Default for vtkOverlappingAMRAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOverlappingAMRAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOverlappingAMRAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOverlappingAMRAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOverlappingAMRAlgorithm_create_drop() {
    let obj = vtkOverlappingAMRAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce output of the same type as input
///
///
/// vtkPassInputTypeAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be DataObject. If that isn't
/// the case then please override this method in your subclass. This class
/// breaks out the downstream requests into separate functions such as
/// RequestDataObject RequestData and RequestInformation. The default
/// implementation of RequestDataObject will create an output data of the
/// same type as the input.
#[allow(non_camel_case_types)]
pub struct vtkPassInputTypeAlgorithm(*mut core::ffi::c_void);
impl vtkPassInputTypeAlgorithm {
    /// Creates a new [vtkPassInputTypeAlgorithm] via `vtkPassInputTypeAlgorithm::New()`
    #[doc(alias = "vtkPassInputTypeAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPassInputTypeAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPassInputTypeAlgorithm_new() })
    }
}
impl std::default::Default for vtkPassInputTypeAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPassInputTypeAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPassInputTypeAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPassInputTypeAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPassInputTypeAlgorithm_create_drop() {
    let obj = vtkPassInputTypeAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only piecewise function as output
///
///
///
/// vtkPiecewiseFunctionAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be PiecewiseFunction. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkPiecewiseFunctionAlgorithm(*mut core::ffi::c_void);
impl vtkPiecewiseFunctionAlgorithm {
    /// Creates a new [vtkPiecewiseFunctionAlgorithm] via `vtkPiecewiseFunctionAlgorithm::New()`
    #[doc(alias = "vtkPiecewiseFunctionAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPiecewiseFunctionAlgorithm_new() })
    }
}
impl std::default::Default for vtkPiecewiseFunctionAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPiecewiseFunctionAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPiecewiseFunctionAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPiecewiseFunctionAlgorithm_create_drop() {
    let obj = vtkPiecewiseFunctionAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
#[allow(non_camel_case_types)]
pub struct vtkPiecewiseFunctionShiftScale(*mut core::ffi::c_void);
impl vtkPiecewiseFunctionShiftScale {
    /// Creates a new [vtkPiecewiseFunctionShiftScale] via `vtkPiecewiseFunctionShiftScale::New()`
    #[doc(alias = "vtkPiecewiseFunctionShiftScale")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionShiftScale_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPiecewiseFunctionShiftScale_new() })
    }
}
impl std::default::Default for vtkPiecewiseFunctionShiftScale {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPiecewiseFunctionShiftScale {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionShiftScale_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPiecewiseFunctionShiftScale_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPiecewiseFunctionShiftScale_create_drop() {
    let obj = vtkPiecewiseFunctionShiftScale::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce output of the same type as input
///
///
/// vtkPointSetAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be PointSet. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkPointSetAlgorithm(*mut core::ffi::c_void);
impl vtkPointSetAlgorithm {
    /// Creates a new [vtkPointSetAlgorithm] via `vtkPointSetAlgorithm::New()`
    #[doc(alias = "vtkPointSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPointSetAlgorithm_new() })
    }
}
impl std::default::Default for vtkPointSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointSetAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointSetAlgorithm_create_drop() {
    let obj = vtkPointSetAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only polydata as output
///
///
///
/// vtkPolyDataAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be PolyData. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkPolyDataAlgorithm(*mut core::ffi::c_void);
impl vtkPolyDataAlgorithm {
    /// Creates a new [vtkPolyDataAlgorithm] via `vtkPolyDataAlgorithm::New()`
    #[doc(alias = "vtkPolyDataAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyDataAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPolyDataAlgorithm_new() })
    }
}
impl std::default::Default for vtkPolyDataAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyDataAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyDataAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyDataAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyDataAlgorithm_create_drop() {
    let obj = vtkPolyDataAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Basic class to optionally replace vtkAlgorithm progress functionality.
///
///
/// When the basic functionality in vtkAlgorithm that reports progress is
/// not enough, a subclass of vtkProgressObserver can be used to provide
/// custom functionality.
/// The main use case for this is when an algorithm's RequestData() is
/// called from multiple threads in parallel - the basic functionality in
/// vtkAlgorithm is not thread safe. vtkSMPProgressObserver can
/// handle this situation by routing progress from each thread to a
/// thread local vtkProgressObserver, which will invoke events separately
/// for each thread.
#[allow(non_camel_case_types)]
pub struct vtkProgressObserver(*mut core::ffi::c_void);
impl vtkProgressObserver {
    /// Creates a new [vtkProgressObserver] via `vtkProgressObserver::New()`
    #[doc(alias = "vtkProgressObserver")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkProgressObserver_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkProgressObserver_new() })
    }
}
impl std::default::Default for vtkProgressObserver {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkProgressObserver {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkProgressObserver_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkProgressObserver_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkProgressObserver_create_drop() {
    let obj = vtkProgressObserver::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Executive that works with vtkReaderAlgorithm and subclasses.
///
///
/// @deprecated VTK 9.1.0. This is no longer needed. vtkReaderAlgorithm can now
/// work with standard executive and hence this can be removed. Follows docs are
/// no longer relevant and left for historical reasons.
///
/// vtkReaderExecutive is an executive that supports simplified API readers
/// that are written by subclassing from the vtkReaderAlgorithm hierarchy.
/// Currently, its main functionality is to call the basic reader API instead
/// if the standard ProcessRequest() method that other algorithms use.
/// In time, this is likely to add functionality such as caching. See
/// vtkReaderAlgorithm for the API.
///
/// Note that this executive assumes that the reader has one output port.
#[allow(non_camel_case_types)]
pub struct vtkReaderExecutive(*mut core::ffi::c_void);
impl vtkReaderExecutive {
    /// Creates a new [vtkReaderExecutive] via `vtkReaderExecutive::New()`
    #[doc(alias = "vtkReaderExecutive")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkReaderExecutive_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkReaderExecutive_new() })
    }
}
impl std::default::Default for vtkReaderExecutive {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkReaderExecutive {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkReaderExecutive_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkReaderExecutive_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkReaderExecutive_create_drop() {
    let obj = vtkReaderExecutive::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only rectilinear grid as output
///
///
///
/// vtkRectilinearGridAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be RectilinearGrid. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkRectilinearGridAlgorithm(*mut core::ffi::c_void);
impl vtkRectilinearGridAlgorithm {
    /// Creates a new [vtkRectilinearGridAlgorithm] via `vtkRectilinearGridAlgorithm::New()`
    #[doc(alias = "vtkRectilinearGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRectilinearGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkRectilinearGridAlgorithm_new() })
    }
}
impl std::default::Default for vtkRectilinearGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRectilinearGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRectilinearGridAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRectilinearGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRectilinearGridAlgorithm_create_drop() {
    let obj = vtkRectilinearGridAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Progress observer that is thread safe
///
///
/// vtkSMPProgressObserver is designed to handle progress events coming
/// from an algorithm in a thread safe way. It does this by using
/// thread local objects that it updates. To receive the progress
/// information, one has to listen to the local observer in the same
/// thread. Since the execution will be somewhat load balanced,
/// it may be enough to do this only on the main thread.
#[allow(non_camel_case_types)]
pub struct vtkSMPProgressObserver(*mut core::ffi::c_void);
impl vtkSMPProgressObserver {
    /// Creates a new [vtkSMPProgressObserver] via `vtkSMPProgressObserver::New()`
    #[doc(alias = "vtkSMPProgressObserver")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSMPProgressObserver_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSMPProgressObserver_new() })
    }
}
impl std::default::Default for vtkSMPProgressObserver {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSMPProgressObserver {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSMPProgressObserver_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSMPProgressObserver_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSMPProgressObserver_create_drop() {
    let obj = vtkSMPProgressObserver::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only Selection as output
///
///
///
/// vtkSelectionAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline edgehitecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Selection. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkSelectionAlgorithm(*mut core::ffi::c_void);
impl vtkSelectionAlgorithm {
    /// Creates a new [vtkSelectionAlgorithm] via `vtkSelectionAlgorithm::New()`
    #[doc(alias = "vtkSelectionAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSelectionAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSelectionAlgorithm_new() })
    }
}
impl std::default::Default for vtkSelectionAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSelectionAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSelectionAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSelectionAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSelectionAlgorithm_create_drop() {
    let obj = vtkSelectionAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// organize data according to scalar values (used to accelerate contouring operations)
///
///
/// vtkSimpleScalarTree creates a pointerless binary tree that helps search
/// for cells that lie within a particular scalar range. This object is used
/// to accelerate some contouring (and other scalar-based techniques).
///
/// The tree consists of an array of (min,max) scalar range pairs per
/// node in the tree. The (min,max) range is determined from looking at
/// the range of the children of the tree node. If the node is a leaf,
/// then the range is determined by scanning the range of scalar data
/// in n cells in the dataset. The n cells are determined by arbitrary
/// selecting cell ids from id(i) to id(i+n), and where n is specified
/// using the BranchingFactor ivar. Note that leaf node i=0 contains
/// the scalar range computed from cell ids (0,n-1); leaf node i=1
/// contains the range from cell ids (n,2n-1); and so on. The
/// implication is that there are no direct lists of cell ids per leaf
/// node, instead the cell ids are implicitly known. Despite the
/// arbitrary grouping of cells, in practice this scalar tree actually
/// performs quite well due to spatial/data coherence.
///
/// This class has an API that supports both serial and parallel
/// operation.  The parallel API enables the using class to grab arrays
/// (or batches) of cells that potentially intersect the
/// isocontour. These batches can then be processed in separate
/// threads.
///
/// @sa
/// vtkScalarTree vtkSpanSpace
#[allow(non_camel_case_types)]
pub struct vtkSimpleScalarTree(*mut core::ffi::c_void);
impl vtkSimpleScalarTree {
    /// Creates a new [vtkSimpleScalarTree] via `vtkSimpleScalarTree::New()`
    #[doc(alias = "vtkSimpleScalarTree")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSimpleScalarTree_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSimpleScalarTree_new() })
    }
}
impl std::default::Default for vtkSimpleScalarTree {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSimpleScalarTree {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSimpleScalarTree_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSimpleScalarTree_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSimpleScalarTree_create_drop() {
    let obj = vtkSimpleScalarTree::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// organize data according to scalar span space
///
///
/// This is a helper class used to accelerate contouring operations. Given an
/// dataset, it organizes the dataset cells into a 2D binned space, with
/// coordinate axes (scalar_min,scalar_max). This so-called span space can
/// then be traversed quickly to find the cells that intersect a specified
/// contour value.
///
/// This class has an API that supports both serial and parallel
/// operation.  The parallel API enables the using class to grab arrays
/// (or batches) of cells that lie along a particular row in the span
/// space. These arrays can then be processed separately or in parallel.
///
/// Learn more about span space in these two publications: 1) "A Near
/// Optimal Isosorface Extraction Algorithm Using the Span Space."
/// Yarden Livnat et al. and 2) Isosurfacing in Span Space with Utmost
/// Efficiency." Han-Wei Shen et al.
///
/// @sa
/// vtkScalarTree vtkSimpleScalarTree
#[allow(non_camel_case_types)]
pub struct vtkSpanSpace(*mut core::ffi::c_void);
impl vtkSpanSpace {
    /// Creates a new [vtkSpanSpace] via `vtkSpanSpace::New()`
    #[doc(alias = "vtkSpanSpace")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSpanSpace_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSpanSpace_new() })
    }
}
impl std::default::Default for vtkSpanSpace {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSpanSpace {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSpanSpace_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSpanSpace_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSpanSpace_create_drop() {
    let obj = vtkSpanSpace::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// class to build and traverse sphere trees
///
///
/// vtkSphereTree is a helper class used to build and traverse sphere
/// trees. Various types of trees can be constructed for different VTK
/// dataset types, as well well as different approaches to organize
/// the tree into hierarchies.
///
/// Typically building a complete sphere tree consists of two parts: 1)
/// creating spheres for each cell in the dataset, then 2) creating an
/// organizing hierarchy. The structure of the hierarchy varies depending on
/// the topological characteristics of the dataset.
///
/// Once the tree is constructed, various geometric operations are available
/// for quickly selecting cells based on sphere tree operations; for example,
/// process all cells intersecting a plane (i.e., use the sphere tree to identify
/// candidate cells for plane intersection).
///
/// This class does not necessarily create optimal sphere trees because
/// some of its requirements (fast build time, provide simple reference
/// code, a single bounding sphere per cell, etc.) precludes optimal
/// performance. It is also oriented to computing on cells versus the
/// classic problem of collision detection for polygonal models. For
/// more information you want to read Gareth Bradshaw's PhD thesis
/// "Bounding Volume Hierarchies for Level-of-Detail Collision
/// Handling" which does a nice job of laying out the challenges and
/// important algorithms relative to sphere trees and BVH (bounding
/// volume hierarchies).
///
/// @sa
/// vtkSphereTreeFilter vtkPlaneCutter
#[allow(non_camel_case_types)]
pub struct vtkSphereTree(*mut core::ffi::c_void);
impl vtkSphereTree {
    /// Creates a new [vtkSphereTree] via `vtkSphereTree::New()`
    #[doc(alias = "vtkSphereTree")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSphereTree_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSphereTree_new() })
    }
}
impl std::default::Default for vtkSphereTree {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSphereTree {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSphereTree_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSphereTree_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSphereTree_create_drop() {
    let obj = vtkSphereTree::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Executive supporting partial updates.
///
///
/// vtkStreamingDemandDrivenPipeline is an executive that supports
/// updating only a portion of the data set in the pipeline.  This is
/// the style of pipeline update that is provided by the old-style VTK
/// 4.x pipeline.  Instead of always updating an entire data set, this
/// executive supports asking for pieces or sub-extents.
#[allow(non_camel_case_types)]
pub struct vtkStreamingDemandDrivenPipeline(*mut core::ffi::c_void);
impl vtkStreamingDemandDrivenPipeline {
    /// Creates a new [vtkStreamingDemandDrivenPipeline] via `vtkStreamingDemandDrivenPipeline::New()`
    #[doc(alias = "vtkStreamingDemandDrivenPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStreamingDemandDrivenPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkStreamingDemandDrivenPipeline_new() })
    }
}
impl std::default::Default for vtkStreamingDemandDrivenPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStreamingDemandDrivenPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStreamingDemandDrivenPipeline_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkStreamingDemandDrivenPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStreamingDemandDrivenPipeline_create_drop() {
    let obj = vtkStreamingDemandDrivenPipeline::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only structured grid as output
///
///
///
/// vtkStructuredGridAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be StructuredGrid. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkStructuredGridAlgorithm(*mut core::ffi::c_void);
impl vtkStructuredGridAlgorithm {
    /// Creates a new [vtkStructuredGridAlgorithm] via `vtkStructuredGridAlgorithm::New()`
    #[doc(alias = "vtkStructuredGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStructuredGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkStructuredGridAlgorithm_new() })
    }
}
impl std::default::Default for vtkStructuredGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStructuredGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStructuredGridAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStructuredGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStructuredGridAlgorithm_create_drop() {
    let obj = vtkStructuredGridAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only vtkTables as output
///
///
///
/// vtkTableAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Tree. If that
/// isn't the case then please override this method in your subclass.
///
/// @par Thanks:
/// Thanks to Brian Wylie for creating this class.
#[allow(non_camel_case_types)]
pub struct vtkTableAlgorithm(*mut core::ffi::c_void);
impl vtkTableAlgorithm {
    /// Creates a new [vtkTableAlgorithm] via `vtkTableAlgorithm::New()`
    #[doc(alias = "vtkTableAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTableAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTableAlgorithm_new() })
    }
}
impl std::default::Default for vtkTableAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTableAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTableAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTableAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTableAlgorithm_create_drop() {
    let obj = vtkTableAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Executive that works in parallel
///
///
/// vtkThreadedCompositeDataPipeline processes a composite data object in
/// parallel using the SMP framework. It does this by creating a vector of
/// data objects (the pieces of the composite data) and processing them
/// using vtkSMPTools::For. Note that this requires that the
/// algorithm implement all pipeline passes in a re-entrant way. It should
/// store/retrieve all state changes using input and output information
/// objects, which are unique to each thread.
#[allow(non_camel_case_types)]
pub struct vtkThreadedCompositeDataPipeline(*mut core::ffi::c_void);
impl vtkThreadedCompositeDataPipeline {
    /// Creates a new [vtkThreadedCompositeDataPipeline] via `vtkThreadedCompositeDataPipeline::New()`
    #[doc(alias = "vtkThreadedCompositeDataPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkThreadedCompositeDataPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkThreadedCompositeDataPipeline_new() })
    }
}
impl std::default::Default for vtkThreadedCompositeDataPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkThreadedCompositeDataPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkThreadedCompositeDataPipeline_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkThreadedCompositeDataPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkThreadedCompositeDataPipeline_create_drop() {
    let obj = vtkThreadedCompositeDataPipeline::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only Tree as output
///
///
///
/// vtkTreeAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline edgehitecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Tree. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkTreeAlgorithm(*mut core::ffi::c_void);
impl vtkTreeAlgorithm {
    /// Creates a new [vtkTreeAlgorithm] via `vtkTreeAlgorithm::New()`
    #[doc(alias = "vtkTreeAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTreeAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTreeAlgorithm_new() })
    }
}
impl std::default::Default for vtkTreeAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTreeAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTreeAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTreeAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTreeAlgorithm_create_drop() {
    let obj = vtkTreeAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Consumer to consume data off of a pipeline.
///
///
/// vtkTrivialConsumer caps off a pipeline so that no output data is left
/// hanging around when a pipeline executes when data is set to be released (see
/// vtkDataObject::SetGlobalReleaseDataFlag). This is intended to be used for
/// tools such as Catalyst and not end users.
#[allow(non_camel_case_types)]
pub struct vtkTrivialConsumer(*mut core::ffi::c_void);
impl vtkTrivialConsumer {
    /// Creates a new [vtkTrivialConsumer] via `vtkTrivialConsumer::New()`
    #[doc(alias = "vtkTrivialConsumer")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTrivialConsumer_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTrivialConsumer_new() })
    }
}
impl std::default::Default for vtkTrivialConsumer {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTrivialConsumer {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTrivialConsumer_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTrivialConsumer_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTrivialConsumer_create_drop() {
    let obj = vtkTrivialConsumer::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Producer for stand-alone data objects.
///
///
/// vtkTrivialProducer allows stand-alone data objects to be connected
/// as inputs in a pipeline.  All data objects that are connected to a
/// pipeline involving vtkAlgorithm must have a producer.  This trivial
/// producer allows data objects that are hand-constructed in a program
/// without another vtk producer to be connected.
#[allow(non_camel_case_types)]
pub struct vtkTrivialProducer(*mut core::ffi::c_void);
impl vtkTrivialProducer {
    /// Creates a new [vtkTrivialProducer] via `vtkTrivialProducer::New()`
    #[doc(alias = "vtkTrivialProducer")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTrivialProducer_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTrivialProducer_new() })
    }
}
impl std::default::Default for vtkTrivialProducer {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTrivialProducer {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTrivialProducer_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTrivialProducer_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTrivialProducer_create_drop() {
    let obj = vtkTrivialProducer::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce undirected graph as output
///
///
///
/// vtkUndirectedGraphAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline edgehitecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Graph. If that
/// isn't the case then please override this method in your subclass.
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkUndirectedGraphAlgorithm(*mut core::ffi::c_void);
impl vtkUndirectedGraphAlgorithm {
    /// Creates a new [vtkUndirectedGraphAlgorithm] via `vtkUndirectedGraphAlgorithm::New()`
    #[doc(alias = "vtkUndirectedGraphAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUndirectedGraphAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUndirectedGraphAlgorithm_new() })
    }
}
impl std::default::Default for vtkUndirectedGraphAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUndirectedGraphAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUndirectedGraphAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUndirectedGraphAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUndirectedGraphAlgorithm_create_drop() {
    let obj = vtkUndirectedGraphAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
/// vtkUniformGridAMR as output.
///
///
/// A base class for all algorithms that take as input any type of data object
/// including composite datasets and produce vtkUniformGridAMR in the output.
#[allow(non_camel_case_types)]
pub struct vtkUniformGridAMRAlgorithm(*mut core::ffi::c_void);
impl vtkUniformGridAMRAlgorithm {
    /// Creates a new [vtkUniformGridAMRAlgorithm] via `vtkUniformGridAMRAlgorithm::New()`
    #[doc(alias = "vtkUniformGridAMRAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformGridAMRAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUniformGridAMRAlgorithm_new() })
    }
}
impl std::default::Default for vtkUniformGridAMRAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformGridAMRAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformGridAMRAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformGridAMRAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformGridAMRAlgorithm_create_drop() {
    let obj = vtkUniformGridAMRAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
///
/// A concrete implementation of vtkMultiBlockDataSetAlgorithm that provides
/// functionality for partitioning a uniform grid. The partitioning method
/// that is used is Recursive Coordinate Bisection (RCB) where each time
/// the longest dimension is split.
///
/// @sa
/// vtkStructuredGridPartitioner vtkRectilinearGridPartitioner
#[allow(non_camel_case_types)]
pub struct vtkUniformGridPartitioner(*mut core::ffi::c_void);
impl vtkUniformGridPartitioner {
    /// Creates a new [vtkUniformGridPartitioner] via `vtkUniformGridPartitioner::New()`
    #[doc(alias = "vtkUniformGridPartitioner")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformGridPartitioner_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUniformGridPartitioner_new() })
    }
}
impl std::default::Default for vtkUniformGridPartitioner {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformGridPartitioner {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformGridPartitioner_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformGridPartitioner_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformGridPartitioner_create_drop() {
    let obj = vtkUniformGridPartitioner::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that produce only unstructured grid as output
///
///
///
/// vtkUnstructuredGridAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be UnstructuredGrid. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkUnstructuredGridAlgorithm(*mut core::ffi::c_void);
impl vtkUnstructuredGridAlgorithm {
    /// Creates a new [vtkUnstructuredGridAlgorithm] via `vtkUnstructuredGridAlgorithm::New()`
    #[doc(alias = "vtkUnstructuredGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnstructuredGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUnstructuredGridAlgorithm_new() })
    }
}
impl std::default::Default for vtkUnstructuredGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnstructuredGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnstructuredGridAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnstructuredGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnstructuredGridAlgorithm_create_drop() {
    let obj = vtkUnstructuredGridAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Superclass for algorithms that
///
/// produce only vtkUnstructureGridBase subclasses as output
///
/// vtkUnstructuredGridBaseAlgorithm is a convenience class to make writing
/// algorithms easier. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be UnstructuredGridBase. If
/// that isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkUnstructuredGridBaseAlgorithm(*mut core::ffi::c_void);
impl vtkUnstructuredGridBaseAlgorithm {
    /// Creates a new [vtkUnstructuredGridBaseAlgorithm] via `vtkUnstructuredGridBaseAlgorithm::New()`
    #[doc(alias = "vtkUnstructuredGridBaseAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnstructuredGridBaseAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUnstructuredGridBaseAlgorithm_new() })
    }
}
impl std::default::Default for vtkUnstructuredGridBaseAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnstructuredGridBaseAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnstructuredGridBaseAlgorithm_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkUnstructuredGridBaseAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnstructuredGridBaseAlgorithm_create_drop() {
    let obj = vtkUnstructuredGridBaseAlgorithm::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
