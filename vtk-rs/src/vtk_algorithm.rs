#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_algorithm.h");

        pub type vtkAlgorithm;
        pub type vtkAlgorithmOutput;
        pub type vtkInformation;
        pub type vtkInformationVector;
        pub type vtkExecutive;
        pub type vtkDataObject;

        fn vtk_algorithm_new() -> *mut vtkAlgorithm;
        fn vtk_algorithm_delete(algorithm: Pin<&mut vtkAlgorithm>);

        fn vtk_algorithm_has_executive(algorithm: &vtkAlgorithm) -> bool;
        fn vtk_algorithm_get_executive(algorithm: &vtkAlgorithm) -> &vtkExecutive;
        fn vtk_algorithm_set_executive(
            algorithm: Pin<&mut vtkAlgorithm>,
            executive: Pin<&mut vtkExecutive>,
        );
        unsafe fn vtk_algorithm_process_request(
            algorithm: &vtkAlgorithm,
            request: &vtkInformation,
            in_info: &mut [*mut vtkInformationVector],
            out_info: Pin<&mut vtkInformationVector>,
        ) -> bool;
        fn vtk_algorithm_modify_request(
            algorithm: &vtkAlgorithm,
            request: Pin<&mut vtkInformation>,
            when: i64,
        ) -> i64;
        fn vtk_algorithm_get_input_port_information(
            algorithm: &vtkAlgorithm,
            port: i64,
        ) -> &vtkInformation;
        fn vtk_algorithm_get_output_port_information(
            algorithm: &vtkAlgorithm,
            port: i64,
        ) -> &vtkInformation;
        fn vtk_algorithm_get_number_of_input_ports(algorithm: &vtkAlgorithm) -> i64;
        fn vtk_algorithm_get_number_of_output_ports(algorithm: &vtkAlgorithm) -> i64;
        fn vtk_algorithm_set_input_connection(
            algorithm: Pin<&mut vtkAlgorithm>,
            port: i64,
            input: &vtkAlgorithmOutput,
        );
        fn vtk_algorithm_set_abort_execute_and_update_time(algorithm: Pin<&mut vtkAlgorithm>);
        fn vtk_algorithm_update_progress(algorithm: Pin<&mut vtkAlgorithm>, amount: f64);
        fn vtk_algorithm_check_abort(algorithm: &vtkAlgorithm) -> bool;
        fn vtk_algorithm_get_input_array_information(
            algorithm: &vtkAlgorithm,
            idx: i64,
        ) -> &vtkInformation;
        fn vtk_algorithm_remove_all_inputs(algorithm: Pin<&mut vtkAlgorithm>);
        fn vtk_algorithm_get_output_data_object(
            algorithm: &vtkAlgorithm,
            port: i64,
        ) -> &vtkDataObject;
        fn vtk_algorithm_get_input_data_object(
            algorithm: &vtkAlgorithm,
            port: i64,
            connection: i64,
        ) -> &vtkDataObject;
        fn vtk_algorithm_remove_input_connection(
            algorithm: Pin<&mut vtkAlgorithm>,
            port: i64,
            input: &vtkAlgorithmOutput,
        );
        fn vtk_algorithm_remove_input_connection_by_idx(
            algorithm: Pin<&mut vtkAlgorithm>,
            port: i64,
            idx: i64,
        );
        fn vtk_algorithm_remove_all_input_connections(algorithm: Pin<&mut vtkAlgorithm>, port: i64);
        fn vtk_algorithm_set_input_data_object(
            algorithm: Pin<&mut vtkAlgorithm>,
            port: i64,
            data: &vtkDataObject,
        );
        fn vtk_algorithm_add_input_data_object(
            algorithm: Pin<&mut vtkAlgorithm>,
            port: i64,
            data: &vtkDataObject,
        );
        fn vtk_algorithm_get_output_port(algorithm: &vtkAlgorithm, idx: i64)
            -> &vtkAlgorithmOutput;
        fn vtk_algorithm_get_number_of_input_connections(
            algorithm: &vtkAlgorithm,
            port: i64,
        ) -> i64;
        fn vtk_algorithm_get_total_number_of_input_connections(algorithm: &vtkAlgorithm) -> i64;
        fn vtk_algorithm_get_input_connection(
            algorithm: &vtkAlgorithm,
            port: i64,
            idx: i64,
        ) -> &vtkAlgorithmOutput;
        fn vtk_algorithm_get_input_algorithm<'a>(
            algorithm: &'a vtkAlgorithm,
            port: i64,
            idx: i64,
            alg_port: &mut i64,
        ) -> &'a vtkAlgorithm;
        fn vtk_algorithm_get_input_executive(
            algorithm: &vtkAlgorithm,
            port: i64,
            idx: i64,
        ) -> &vtkExecutive;
        fn vtk_algorithm_get_input_information(
            algorithm: &vtkAlgorithm,
            port: i64,
            idx: i64,
        ) -> &vtkInformation;
        fn vtk_algorithm_get_output_information(
            algorithm: &vtkAlgorithm,
            port: i64,
        ) -> &vtkInformation;
        fn vtk_algorithm_update(
            algorithm: Pin<&mut vtkAlgorithm>,
            port: i64,
            requests: &vtkInformationVector,
        ) -> bool;
        // TODO fn vtk_algorithm_update_piece(algorithm: &vtkAlgorithm, piece: i64, num_pieces: int, ghost_levels: i64, extent: Option<[int; 6]>);
        // TODO fn vtk_algorithm_update_time_step(algorithm: &vtkAlgorithm, time: f64, piece: i64 = -1, num_pieces: i64 = -1, ghost_levels: i64 = 0, extents: [i64; 6] = None);
        fn vtk_algorithm_update_information(algorithm: Pin<&mut vtkAlgorithm>);
        fn vtk_algorithm_update_data_object(algorithm: Pin<&mut vtkAlgorithm>);
        fn vtk_algorithm_propagate_update_extent(algorithm: Pin<&mut vtkAlgorithm>);
        fn vtk_algorithm_update_whole_extent(algorithm: Pin<&mut vtkAlgorithm>);
        fn vtk_algorithm_convert_total_input_to_port_connection(
            algorithm: &vtkAlgorithm,
            ind: i64,
        ) -> [i64; 2];
        fn vtk_algorithm_remove_no_prior_temporal_access_information_key(
            algorithm: Pin<&mut vtkAlgorithm>,
        );
        fn vtk_algorithm_get_information(algorithm: &vtkAlgorithm) -> &vtkInformation;
        fn vtk_algorithm_set_information(
            algorithm: Pin<&mut vtkAlgorithm>,
            information: &vtkInformation,
        );
        fn vtk_algorithm_get_abort_execute(algorithm: &vtkAlgorithm) -> bool;
        fn vtk_algorithm_abort_execute_on(algorithm: Pin<&mut vtkAlgorithm>);
        fn vtk_algorithm_abort_execute_off(algorithm: Pin<&mut vtkAlgorithm>);
        fn vtk_algorithm_get_progress(algorithm: &vtkAlgorithm) -> f64;
        fn vtk_algorithm_set_container_algorithm(
            algorithm: Pin<&mut vtkAlgorithm>,
            container_algorithm: &vtkAlgorithm,
        );
        fn vtk_algorithm_get_container_algorithm(algorithm: &vtkAlgorithm) -> &vtkAlgorithm;
        fn vtk_algorithm_set_abort_output(algorithm: Pin<&mut vtkAlgorithm>, flag: bool);
        fn vtk_algorithm_get_abort_output(algorithm: &vtkAlgorithm) -> bool;
        fn vtk_algorithm_set_progress_shift_scale(
            algorithm: Pin<&mut vtkAlgorithm>,
            shift: f64,
            scale: f64,
        );
        fn vtk_algorithm_get_progress_shift(algorithm: &vtkAlgorithm) -> f64;
        fn vtk_algorithm_get_progress_scale(algorithm: &vtkAlgorithm) -> f64;
        fn vtk_algorithm_set_progress_text(algorithm: Pin<&mut vtkAlgorithm>, ptext: &str);
        fn vtk_algorithm_get_progress_text(algorithm: &vtkAlgorithm) -> String;
        fn vtk_algorithm_get_error_code(algorithm: &vtkAlgorithm) -> u64;
        fn vtk_algorithm_set_input_array_to_process(
            algorithm: Pin<&mut vtkAlgorithm>,
            idx: i64,
            port: i64,
            connection: i64,
            field_association: &str,
            attribute_type_or_name: &str,
        );
        fn vtk_algorithm_add_input_connection(
            algorithm: Pin<&mut vtkAlgorithm>,
            port: i64,
            input: &vtkAlgorithmOutput,
        );
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAlgorithm.html",
    @name Algorithm, ffi::vtkAlgorithm,
    @new ffi::vtk_algorithm_new,
    @delete ffi::vtk_algorithm_delete,
    @inherit vtkAlgorithm
);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkAlgorithm`](https://vtk.org/doc/nightly/html/classvtkAlgorithm.html)
#[allow(non_camel_case_types)]
pub trait vtkAlgorithm: private::Sealed {
    #[doc(hidden)]
    fn as_vtk_algorithm(&self) -> core::pin::Pin<&ffi::vtkAlgorithm>;
    #[doc(hidden)]
    fn as_vtk_algorithm_mut(&mut self) -> core::pin::Pin<&mut ffi::vtkAlgorithm>;

    fn has_executive(&self) -> bool {
        ffi::vtk_algorithm_has_executive(&self.as_vtk_algorithm())
    }

    fn get_executive(&self) -> &crate::vtk_executive::Executive {
        let x = ffi::vtk_algorithm_get_executive(&self.as_vtk_algorithm()) as *const _
            as *const crate::vtk_executive::Executive;
        unsafe { &*x }
    }

    fn set_executive(&mut self, executive: &mut impl crate::vtk_executive::vtkExecutive) {
        let executive = unsafe {
            executive
                .as_vtk_executive_mut()
                .map_unchecked_mut(|x| &mut *(x as *mut _ as *mut ffi::vtkExecutive))
        };
        let sself = self.as_vtk_algorithm_mut();
        ffi::vtk_algorithm_set_executive(sself, executive);
    }

    fn vtk_algorithm_process_request(
        &self,
        request: &crate::vtk_information::Information,
        in_info: &mut [*mut crate::vtk_information_vector::InformationVector],
        out_info: core::pin::Pin<&mut crate::vtk_information_vector::InformationVector>,
    ) -> bool {
        let sself = self.as_vtk_algorithm();
        let request = unsafe { &*(request as *const _ as *const ffi::vtkInformation) };
        let mut in_info = in_info
            .iter_mut()
            .map(|x| x as *mut _ as *mut ffi::vtkInformationVector)
            .collect::<Vec<_>>();
        let out_info = unsafe {
            out_info.map_unchecked_mut(|x| &mut *(x as *mut _ as *mut ffi::vtkInformationVector))
        };
        unsafe { ffi::vtk_algorithm_process_request(&sself, request, &mut in_info, out_info) }
    }

    fn modify_request(&self, request: &mut crate::vtk_information::Information, when: i64) -> i64 {
        let sself = self.as_vtk_algorithm();
        let request = unsafe {
            request
                .as_vtk_information_mut()
                .map_unchecked_mut(|x| &mut *(x as *mut _ as *mut ffi::vtkInformation))
        };
        ffi::vtk_algorithm_modify_request(&sself, request, when)
    }

    fn get_input_port_information(
        &self,
        port: i64,
    ) -> Option<&crate::vtk_information::Information> {
        let sself = self.as_vtk_algorithm();
        unsafe {
            (ffi::vtk_algorithm_get_input_port_information(&sself, port) as *const _
                as *const crate::vtk_information::Information)
                .as_ref()
        }
    }

    fn get_output_port_information(
        &self,
        port: i64,
    ) -> Option<&crate::vtk_information::Information> {
        let sself = self.as_vtk_algorithm();
        unsafe {
            (ffi::vtk_algorithm_get_output_port_information(&sself, port) as *const _
                as *const crate::vtk_information::Information)
                .as_ref()
        }
    }

    fn get_number_of_input_ports(&self) -> i64 {
        ffi::vtk_algorithm_get_number_of_input_ports(&self.as_vtk_algorithm())
    }

    fn get_number_of_output_ports(&self) -> i64 {
        ffi::vtk_algorithm_get_number_of_output_ports(&self.as_vtk_algorithm())
    }

    fn set_input_connection(&mut self, port: i64, input: &impl crate::vtkAlgorithmOutput) {
        let sself = self.as_vtk_algorithm_mut();
        let input = &input.as_vtk_algorithm_output() as *const _ as *const ffi::vtkAlgorithmOutput;
        ffi::vtk_algorithm_set_input_connection(sself, port, unsafe { &*input });
    }

    #[cfg(feature = "v094")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v094")))]
    fn set_abort_execute_and_update_time(&mut self) {
        ffi::vtk_algorithm_set_abort_execute_and_update_time(self.as_vtk_algorithm_mut())
    }

    fn update_progress(&mut self, amount: f64) {
        ffi::vtk_algorithm_update_progress(self.as_vtk_algorithm_mut(), amount)
    }

    #[cfg(feature = "v094")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v094")))]
    fn check_abort(&self) -> bool {
        ffi::vtk_algorithm_check_abort(&self.as_vtk_algorithm())
    }

    fn vtk_algorithm_get_input_array_information(
        &self,
        idx: i64,
    ) -> Option<&crate::vtk_information::Information> {
        let sself = self.as_vtk_algorithm();
        let info = ffi::vtk_algorithm_get_input_array_information(&sself, idx);
        let info = info as *const _ as *const crate::vtk_information::Information;
        unsafe { info.as_ref() }
    }

    fn remove_all_inputs(&mut self) {
        ffi::vtk_algorithm_remove_all_inputs(self.as_vtk_algorithm_mut())
    }

    fn vtk_algorithm_get_output_data_object(
        &self,
        port: i64,
    ) -> Option<&crate::vtk_data_object::DataObject> {
        let sself = self.as_vtk_algorithm();
        let dobject = ffi::vtk_algorithm_get_output_data_object(&sself, port);
        unsafe { (dobject as *const _ as *const crate::vtk_data_object::DataObject).as_ref() }
    }

    fn vtk_algorithm_get_input_data_object(
        &self,
        port: i64,
        connection: i64,
    ) -> Option<&crate::vtk_data_object::DataObject> {
        let sself = self.as_vtk_algorithm();
        let dobject = ffi::vtk_algorithm_get_input_data_object(&sself, port, connection);
        unsafe { (dobject as *const _ as *const crate::vtk_data_object::DataObject).as_ref() }
    }

    fn remove_input_connection(
        &mut self,
        port: i64,
        input: &impl crate::vtk_algorithm_output::vtkAlgorithmOutput,
    ) {
        let input = &input.as_vtk_algorithm_output() as *const _ as *const ffi::vtkAlgorithmOutput;
        ffi::vtk_algorithm_remove_input_connection(self.as_vtk_algorithm_mut(), port, unsafe {
            &*input
        });
    }

    fn remove_input_connection_by_idx(&mut self, port: i64, idx: i64) {
        ffi::vtk_algorithm_remove_input_connection_by_idx(self.as_vtk_algorithm_mut(), port, idx)
    }

    fn remove_all_input_connections(&mut self, port: i64) {
        ffi::vtk_algorithm_remove_all_input_connections(self.as_vtk_algorithm_mut(), port)
    }

    fn set_input_data_object(
        &mut self,
        port: i64,
        data: &impl crate::vtk_data_object::vtkDataObject,
    ) {
        let data = &data.as_vtk_data_object() as *const _ as *const ffi::vtkDataObject;
        ffi::vtk_algorithm_set_input_data_object(self.as_vtk_algorithm_mut(), port, unsafe {
            &*data
        });
    }

    fn add_input_data_object(
        &mut self,
        port: i64,
        data: &impl crate::vtk_data_object::vtkDataObject,
    ) {
        let data = &data.as_vtk_data_object() as *const _ as *const ffi::vtkDataObject;
        ffi::vtk_algorithm_add_input_data_object(self.as_vtk_algorithm_mut(), port, unsafe {
            &*data
        });
    }

    fn get_output_port(&self, idx: i64) -> Option<&crate::vtk_algorithm_output::AlgorithmOutput> {
        let sself = &self.as_vtk_algorithm();
        let out = ffi::vtk_algorithm_get_output_port(sself, idx);
        unsafe { (out as *const _ as *const crate::vtk_algorithm_output::AlgorithmOutput).as_ref() }
    }

    fn get_number_of_input_connections(&self, port: i64) -> i64 {
        ffi::vtk_algorithm_get_number_of_input_connections(&self.as_vtk_algorithm(), port)
    }

    fn get_total_number_of_input_connections(&self) -> i64 {
        ffi::vtk_algorithm_get_total_number_of_input_connections(&self.as_vtk_algorithm())
    }

    fn get_input_connection(
        &mut self,
        port: i64,
        idx: i64,
    ) -> Option<&crate::vtk_algorithm_output::AlgorithmOutput> {
        let sself = self.as_vtk_algorithm();
        let out = ffi::vtk_algorithm_get_input_connection(&sself, port, idx);
        unsafe { (out as *const _ as *const crate::vtk_algorithm_output::AlgorithmOutput).as_ref() }
    }

    fn get_input_algorithm(&self, port: i64, idx: i64) -> i64 {
        let mut algo_port = 0;
        let sself = self.as_vtk_algorithm();
        ffi::vtk_algorithm_get_input_algorithm(&sself, port, idx, &mut algo_port);
        algo_port
    }

    fn get_input_executive(&self, port: i64, idx: i64) -> Option<&crate::vtk_executive::Executive> {
        let sself = self.as_vtk_algorithm();
        let exec = ffi::vtk_algorithm_get_input_executive(&sself, port, idx);
        unsafe { (exec as *const _ as *const crate::vtk_executive::Executive).as_ref() }
    }

    fn get_input_information(
        &self,
        port: i64,
        idx: i64,
    ) -> Option<&crate::vtk_information::Information> {
        let sself = self.as_vtk_algorithm();
        let info = ffi::vtk_algorithm_get_input_information(&sself, port, idx);
        unsafe { (info as *const _ as *const crate::vtk_information::Information).as_ref() }
    }

    fn get_output_information(&self, port: i64) -> Option<&crate::vtk_information::Information> {
        let sself = self.as_vtk_algorithm();
        let info = ffi::vtk_algorithm_get_output_information(&sself, port);
        unsafe { (info as *const _ as *const crate::vtk_information::Information).as_ref() }
    }

    fn update(
        &mut self,
        port: i64,
        requests: &crate::vtk_information_vector::InformationVector,
    ) -> bool {
        let requests = unsafe { &*(requests as *const _ as *const ffi::vtkInformationVector) };
        ffi::vtk_algorithm_update(self.as_vtk_algorithm_mut(), port, requests)
    }

    fn update_information(&mut self) {
        ffi::vtk_algorithm_update_information(self.as_vtk_algorithm_mut())
    }

    fn update_data_object(&mut self) {
        ffi::vtk_algorithm_update_data_object(self.as_vtk_algorithm_mut())
    }

    fn propagate_update_extent(&mut self) {
        ffi::vtk_algorithm_propagate_update_extent(self.as_vtk_algorithm_mut())
    }

    fn update_whole_extent(&mut self) {
        ffi::vtk_algorithm_update_whole_extent(self.as_vtk_algorithm_mut())
    }

    fn convert_total_input_to_port_connection(&self, ind: i64) -> (i64, i64) {
        let sself = self.as_vtk_algorithm();
        let [port, connection] =
            ffi::vtk_algorithm_convert_total_input_to_port_connection(&sself, ind);
        (port, connection)
    }

    #[cfg(feature = "v094")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v094")))]
    fn remove_no_prior_temporal_access_information_key(&mut self) {
        ffi::vtk_algorithm_remove_no_prior_temporal_access_information_key(
            self.as_vtk_algorithm_mut(),
        )
    }

    fn get_information(&self) -> Option<&crate::vtk_information::Information> {
        let sself = self.as_vtk_algorithm();
        let info = ffi::vtk_algorithm_get_information(&sself);
        unsafe { (info as *const _ as *const crate::vtk_information::Information).as_ref() }
    }

    fn set_information(&mut self, information: &crate::vtk_information::Information) {
        let info = unsafe { &*(information as *const _ as *const ffi::vtkInformation) };
        ffi::vtk_algorithm_set_information(self.as_vtk_algorithm_mut(), info)
    }

    fn get_abort_execute(&self) -> bool {
        let sself = self.as_vtk_algorithm();
        ffi::vtk_algorithm_get_abort_execute(&sself)
    }

    fn abort_execute_on(&mut self) {
        ffi::vtk_algorithm_abort_execute_on(self.as_vtk_algorithm_mut())
    }

    fn abort_execute_off(&mut self) {
        ffi::vtk_algorithm_abort_execute_off(self.as_vtk_algorithm_mut())
    }

    fn get_progress(&self) -> f64 {
        ffi::vtk_algorithm_get_progress(&self.as_vtk_algorithm())
    }

    #[cfg(feature = "v094")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v094")))]
    fn set_container_algorithm(&mut self, container_algorithm: &impl vtkAlgorithm) {
        let container_algorithm = container_algorithm.as_vtk_algorithm();
        ffi::vtk_algorithm_set_container_algorithm(
            self.as_vtk_algorithm_mut(),
            &container_algorithm,
        )
    }

    #[cfg(feature = "v094")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v094")))]
    fn get_container_algorithm(&self) -> Option<&crate::vtk_algorithm::Algorithm> {
        let sself = self.as_vtk_algorithm();
        let algo = ffi::vtk_algorithm_get_container_algorithm(&sself);
        unsafe { (algo as *const _ as *const crate::vtk_algorithm::Algorithm).as_ref() }
    }

    #[cfg(feature = "v094")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v094")))]
    fn set_abort_output(&mut self, flag: bool) {
        ffi::vtk_algorithm_set_abort_output(self.as_vtk_algorithm_mut(), flag)
    }

    #[cfg(feature = "v094")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v094")))]
    fn get_abort_output(&self) -> bool {
        ffi::vtk_algorithm_get_abort_output(&self.as_vtk_algorithm())
    }

    fn set_progress_shift_scale(&mut self, shift: f64, scale: f64) {
        ffi::vtk_algorithm_set_progress_shift_scale(self.as_vtk_algorithm_mut(), shift, scale)
    }

    fn get_progress_shift(&self) -> f64 {
        ffi::vtk_algorithm_get_progress_shift(&self.as_vtk_algorithm())
    }

    fn get_progress_scale(&self) -> f64 {
        ffi::vtk_algorithm_get_progress_scale(&self.as_vtk_algorithm())
    }

    fn set_progress_text(&mut self, ptext: &str) {
        ffi::vtk_algorithm_set_progress_text(self.as_vtk_algorithm_mut(), ptext)
    }

    fn get_progress_text(&self) -> String {
        ffi::vtk_algorithm_get_progress_text(&self.as_vtk_algorithm())
    }

    fn get_error_code(&self) -> u64 {
        ffi::vtk_algorithm_get_error_code(&self.as_vtk_algorithm())
    }

    fn set_input_array_to_process(
        &mut self,
        idx: i64,
        port: i64,
        connection: i64,
        field_association: &str,
        attribute_type_or_name: &str,
    ) {
        ffi::vtk_algorithm_set_input_array_to_process(
            self.as_vtk_algorithm_mut(),
            idx,
            port,
            connection,
            field_association,
            attribute_type_or_name,
        )
    }

    fn add_input_connection(
        &mut self,
        port: i64,
        input: &impl crate::vtk_algorithm_output::vtkAlgorithmOutput,
    ) {
        let input = &input.as_vtk_algorithm_output() as *const _ as *const ffi::vtkAlgorithmOutput;
        ffi::vtk_algorithm_add_input_connection(self.as_vtk_algorithm_mut(), port, unsafe {
            &*input
        })
    }
}
