// Include header file
#include<vtk_common_execution_model.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkAlgorithm.h>
#include<vtkAlgorithmOutput.h>
#include<vtkAnnotationLayersAlgorithm.h>
#include<vtkArrayDataAlgorithm.h>
#include<vtkCachedStreamingDemandDrivenPipeline.h>
#include<vtkCastToConcrete.h>
#include<vtkCompositeDataPipeline.h>
#include<vtkCompositeDataSetAlgorithm.h>
#include<vtkDataObjectAlgorithm.h>
#include<vtkDataSetAlgorithm.h>
#include<vtkDemandDrivenPipeline.h>
#include<vtkDirectedGraphAlgorithm.h>
#include<vtkEnsembleSource.h>
#include<vtkExecutive.h>
#include<vtkExplicitStructuredGridAlgorithm.h>
#include<vtkExtentRCBPartitioner.h>
#include<vtkExtentSplitter.h>
#include<vtkExtentTranslator.h>
#include<vtkFilteringInformationKeyManager.h>
#include<vtkGraphAlgorithm.h>
#include<vtkHierarchicalBoxDataSetAlgorithm.h>
#include<vtkHyperTreeGridAlgorithm.h>
#include<vtkImageAlgorithm.h>
#include<vtkImageInPlaceFilter.h>
#include<vtkImageProgressIterator.h>
#include<vtkImageToStructuredGrid.h>
#include<vtkImageToStructuredPoints.h>
#include<vtkInformationDataObjectMetaDataKey.h>
#include<vtkInformationExecutivePortKey.h>
#include<vtkInformationExecutivePortVectorKey.h>
#include<vtkInformationIntegerRequestKey.h>
#include<vtkMoleculeAlgorithm.h>
#include<vtkMultiBlockDataSetAlgorithm.h>
#include<vtkMultiTimeStepAlgorithm.h>
#include<vtkNonOverlappingAMRAlgorithm.h>
#include<vtkOverlappingAMRAlgorithm.h>
#include<vtkParallelReader.h>
#include<vtkPartitionedDataSetAlgorithm.h>
#include<vtkPartitionedDataSetCollectionAlgorithm.h>
#include<vtkPassInputTypeAlgorithm.h>
#include<vtkPiecewiseFunctionAlgorithm.h>
#include<vtkPiecewiseFunctionShiftScale.h>
#include<vtkPointSetAlgorithm.h>
#include<vtkPolyDataAlgorithm.h>
#include<vtkProgressObserver.h>
#include<vtkReaderAlgorithm.h>
#include<vtkReaderExecutive.h>
#include<vtkRectilinearGridAlgorithm.h>
#include<vtkSMPProgressObserver.h>
#include<vtkScalarTree.h>
#include<vtkSelectionAlgorithm.h>
#include<vtkSimpleImageToImageFilter.h>
#include<vtkSimpleReader.h>
#include<vtkSimpleScalarTree.h>
#include<vtkSpanSpace.h>
#include<vtkSphereTree.h>
#include<vtkStreamingDemandDrivenPipeline.h>
#include<vtkStructuredGridAlgorithm.h>
#include<vtkTableAlgorithm.h>
#include<vtkThreadedCompositeDataPipeline.h>
#include<vtkThreadedImageAlgorithm.h>
#include<vtkTreeAlgorithm.h>
#include<vtkTrivialConsumer.h>
#include<vtkTrivialProducer.h>
#include<vtkUndirectedGraphAlgorithm.h>
#include<vtkUniformGridAMRAlgorithm.h>
#include<vtkUniformGridPartitioner.h>
#include<vtkUnstructuredGridAlgorithm.h>
#include<vtkUnstructuredGridBaseAlgorithm.h>

// Implement declared functions
extern "C" vtkAlgorithm * vtkAlgorithm_new () {return vtkAlgorithm :: New () ;}
extern "C" void vtkAlgorithm_destructor (vtkAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_algorithm_has_executive(vtkAlgorithm* sself) { return sself->HasExecutive(); }
extern "C" int vtk_algorithm_get_number_of_input_ports(vtkAlgorithm* sself) { return sself->GetNumberOfInputPorts(); }
extern "C" int vtk_algorithm_get_number_of_output_ports(vtkAlgorithm* sself) { return sself->GetNumberOfOutputPorts(); }
extern "C" void vtk_algorithm_set_abort_execute(vtkAlgorithm* sself, int _arg) { sself->SetAbortExecute(_arg); }
extern "C" int vtk_algorithm_get_abort_execute(vtkAlgorithm* sself) { return sself->GetAbortExecute(); }
extern "C" void vtk_algorithm_abort_execute_on(vtkAlgorithm* sself) { sself->AbortExecuteOn(); }
extern "C" void vtk_algorithm_abort_execute_off(vtkAlgorithm* sself) { sself->AbortExecuteOff(); }
extern "C" double vtk_algorithm_get_progress(vtkAlgorithm* sself) { return sself->GetProgress(); }
extern "C" void vtk_algorithm_set_progress(vtkAlgorithm* sself, double p0) { sself->SetProgress(p0); }
extern "C" void vtk_algorithm_update_progress(vtkAlgorithm* sself, double amount) { sself->UpdateProgress(amount); }
extern "C" void vtk_algorithm_set_progress_shift_scale(vtkAlgorithm* sself, double shift, double scale) { sself->SetProgressShiftScale(shift, scale); }
extern "C" double vtk_algorithm_get_progress_shift(vtkAlgorithm* sself) { return sself->GetProgressShift(); }
extern "C" double vtk_algorithm_get_progress_scale(vtkAlgorithm* sself) { return sself->GetProgressScale(); }
extern "C" void vtk_algorithm_set_progress_text(vtkAlgorithm* sself, const char* ptext) { sself->SetProgressText(ptext); }
extern "C" unsigned long vtk_algorithm_get_error_code(vtkAlgorithm* sself) { return sself->GetErrorCode(); }
extern "C" void vtk_algorithm_set_input_array_to_process(vtkAlgorithm* sself, int idx, int port, int connection, int fieldAssociation, const char* name) { sself->SetInputArrayToProcess(idx, port, connection, fieldAssociation, name); }
extern "C" void vtk_algorithm_remove_all_inputs(vtkAlgorithm* sself) { sself->RemoveAllInputs(); }
extern "C" void vtk_algorithm_remove_all_input_connections(vtkAlgorithm* sself, int port) { sself->RemoveAllInputConnections(port); }
extern "C" int vtk_algorithm_get_number_of_input_connections(vtkAlgorithm* sself, int port) { return sself->GetNumberOfInputConnections(port); }
extern "C" int vtk_algorithm_get_total_number_of_input_connections(vtkAlgorithm* sself) { return sself->GetTotalNumberOfInputConnections(); }
extern "C" void vtk_algorithm_update(vtkAlgorithm* sself, int port) { sself->Update(port); }
extern "C" void vtk_algorithm_update_information(vtkAlgorithm* sself) { sself->UpdateInformation(); }
extern "C" void vtk_algorithm_update_data_object(vtkAlgorithm* sself) { sself->UpdateDataObject(); }
extern "C" void vtk_algorithm_propagate_update_extent(vtkAlgorithm* sself) { sself->PropagateUpdateExtent(); }
extern "C" void vtk_algorithm_update_whole_extent(vtkAlgorithm* sself) { sself->UpdateWholeExtent(); }
extern "C" void vtk_algorithm_convert_total_input_to_port_connection(vtkAlgorithm* sself, int ind, int& port, int& conn) { sself->ConvertTotalInputToPortConnection(ind, port, conn); }
extern "C" void vtk_algorithm_set_release_data_flag(vtkAlgorithm* sself, int p0) { sself->SetReleaseDataFlag(p0); }
extern "C" int vtk_algorithm_get_release_data_flag(vtkAlgorithm* sself) { return sself->GetReleaseDataFlag(); }
extern "C" void vtk_algorithm_release_data_flag_on(vtkAlgorithm* sself) { sself->ReleaseDataFlagOn(); }
extern "C" void vtk_algorithm_release_data_flag_off(vtkAlgorithm* sself) { sself->ReleaseDataFlagOff(); }
extern "C" int vtk_algorithm_get_update_piece(vtkAlgorithm* sself) { return sself->GetUpdatePiece(); }
extern "C" int vtk_algorithm_get_update_number_of_pieces(vtkAlgorithm* sself) { return sself->GetUpdateNumberOfPieces(); }
extern "C" int vtk_algorithm_get_update_ghost_level(vtkAlgorithm* sself) { return sself->GetUpdateGhostLevel(); }
extern "C" vtkAlgorithmOutput * vtkAlgorithmOutput_new () {return vtkAlgorithmOutput :: New () ;}
extern "C" void vtkAlgorithmOutput_destructor (vtkAlgorithmOutput * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_algorithm_output_set_index(vtkAlgorithmOutput* sself, int index) { sself->SetIndex(index); }
extern "C" int vtk_algorithm_output_get_index(vtkAlgorithmOutput* sself) { return sself->GetIndex(); }
extern "C" vtkAnnotationLayersAlgorithm * vtkAnnotationLayersAlgorithm_new () {return vtkAnnotationLayersAlgorithm :: New () ;}
extern "C" void vtkAnnotationLayersAlgorithm_destructor (vtkAnnotationLayersAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkArrayDataAlgorithm * vtkArrayDataAlgorithm_new () {return vtkArrayDataAlgorithm :: New () ;}
extern "C" void vtkArrayDataAlgorithm_destructor (vtkArrayDataAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkCachedStreamingDemandDrivenPipeline * vtkCachedStreamingDemandDrivenPipeline_new () {return vtkCachedStreamingDemandDrivenPipeline :: New () ;}
extern "C" void vtkCachedStreamingDemandDrivenPipeline_destructor (vtkCachedStreamingDemandDrivenPipeline * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cached_streaming_demand_driven_pipeline_set_cache_size(vtkCachedStreamingDemandDrivenPipeline* sself, int size) { sself->SetCacheSize(size); }
extern "C" int vtk_cached_streaming_demand_driven_pipeline_get_cache_size(vtkCachedStreamingDemandDrivenPipeline* sself) { return sself->GetCacheSize(); }
extern "C" vtkCastToConcrete * vtkCastToConcrete_new () {return vtkCastToConcrete :: New () ;}
extern "C" void vtkCastToConcrete_destructor (vtkCastToConcrete * sself) {sself -> Delete () ; return ;}
extern "C" vtkCompositeDataPipeline * vtkCompositeDataPipeline_new () {return vtkCompositeDataPipeline :: New () ;}
extern "C" void vtkCompositeDataPipeline_destructor (vtkCompositeDataPipeline * sself) {sself -> Delete () ; return ;}
extern "C" vtkCompositeDataSetAlgorithm * vtkCompositeDataSetAlgorithm_new () {return vtkCompositeDataSetAlgorithm :: New () ;}
extern "C" void vtkCompositeDataSetAlgorithm_destructor (vtkCompositeDataSetAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkDataObjectAlgorithm * vtkDataObjectAlgorithm_new () {return vtkDataObjectAlgorithm :: New () ;}
extern "C" void vtkDataObjectAlgorithm_destructor (vtkDataObjectAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkDataSetAlgorithm * vtkDataSetAlgorithm_new () {return vtkDataSetAlgorithm :: New () ;}
extern "C" void vtkDataSetAlgorithm_destructor (vtkDataSetAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkDemandDrivenPipeline * vtkDemandDrivenPipeline_new () {return vtkDemandDrivenPipeline :: New () ;}
extern "C" void vtkDemandDrivenPipeline_destructor (vtkDemandDrivenPipeline * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_demand_driven_pipeline_get_pipeline_m_time(vtkDemandDrivenPipeline* sself) { return sself->GetPipelineMTime(); }
extern "C" int vtk_demand_driven_pipeline_set_release_data_flag(vtkDemandDrivenPipeline* sself, int port, int n) { return sself->SetReleaseDataFlag(port, n); }
extern "C" int vtk_demand_driven_pipeline_get_release_data_flag(vtkDemandDrivenPipeline* sself, int port) { return sself->GetReleaseDataFlag(port); }
extern "C" int vtk_demand_driven_pipeline_update_pipeline_m_time(vtkDemandDrivenPipeline* sself) { return sself->UpdatePipelineMTime(); }
extern "C" int vtk_demand_driven_pipeline_update_data_object(vtkDemandDrivenPipeline* sself) { return sself->UpdateDataObject(); }
extern "C" int vtk_demand_driven_pipeline_update_data(vtkDemandDrivenPipeline* sself, int outputPort) { return sself->UpdateData(outputPort); }
extern "C" vtkDirectedGraphAlgorithm * vtkDirectedGraphAlgorithm_new () {return vtkDirectedGraphAlgorithm :: New () ;}
extern "C" void vtkDirectedGraphAlgorithm_destructor (vtkDirectedGraphAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkEnsembleSource * vtkEnsembleSource_new () {return vtkEnsembleSource :: New () ;}
extern "C" void vtkEnsembleSource_destructor (vtkEnsembleSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_ensemble_source_remove_all_members(vtkEnsembleSource* sself) { sself->RemoveAllMembers(); }
extern "C" unsigned int vtk_ensemble_source_get_number_of_members(vtkEnsembleSource* sself) { return sself->GetNumberOfMembers(); }
extern "C" void vtk_ensemble_source_set_current_member(vtkEnsembleSource* sself, unsigned int _arg) { sself->SetCurrentMember(_arg); }
extern "C" unsigned int vtk_ensemble_source_get_current_member(vtkEnsembleSource* sself) { return sself->GetCurrentMember(); }
extern "C" vtkExplicitStructuredGridAlgorithm * vtkExplicitStructuredGridAlgorithm_new () {return vtkExplicitStructuredGridAlgorithm :: New () ;}
extern "C" void vtkExplicitStructuredGridAlgorithm_destructor (vtkExplicitStructuredGridAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkExtentRCBPartitioner * vtkExtentRCBPartitioner_new () {return vtkExtentRCBPartitioner :: New () ;}
extern "C" void vtkExtentRCBPartitioner_destructor (vtkExtentRCBPartitioner * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_extent_rcb_partitioner_set_number_of_partitions(vtkExtentRCBPartitioner* sself, const int N) { sself->SetNumberOfPartitions(N); }
extern "C" void vtk_extent_rcb_partitioner_set_global_extent(vtkExtentRCBPartitioner* sself, int imin, int imax, int jmin, int jmax, int kmin, int kmax) { sself->SetGlobalExtent(imin, imax, jmin, jmax, kmin, kmax); }
extern "C" void vtk_extent_rcb_partitioner_set_duplicate_nodes(vtkExtentRCBPartitioner* sself, int _arg) { sself->SetDuplicateNodes(_arg); }
extern "C" int vtk_extent_rcb_partitioner_get_duplicate_nodes(vtkExtentRCBPartitioner* sself) { return sself->GetDuplicateNodes(); }
extern "C" void vtk_extent_rcb_partitioner_duplicate_nodes_on(vtkExtentRCBPartitioner* sself) { sself->DuplicateNodesOn(); }
extern "C" void vtk_extent_rcb_partitioner_duplicate_nodes_off(vtkExtentRCBPartitioner* sself) { sself->DuplicateNodesOff(); }
extern "C" void vtk_extent_rcb_partitioner_set_number_of_ghost_layers(vtkExtentRCBPartitioner* sself, int _arg) { sself->SetNumberOfGhostLayers(_arg); }
extern "C" int vtk_extent_rcb_partitioner_get_number_of_ghost_layers(vtkExtentRCBPartitioner* sself) { return sself->GetNumberOfGhostLayers(); }
extern "C" int vtk_extent_rcb_partitioner_get_num_extents(vtkExtentRCBPartitioner* sself) { return sself->GetNumExtents(); }
extern "C" void vtk_extent_rcb_partitioner_partition(vtkExtentRCBPartitioner* sself) { sself->Partition(); }
extern "C" vtkExtentSplitter * vtkExtentSplitter_new () {return vtkExtentSplitter :: New () ;}
extern "C" void vtkExtentSplitter_destructor (vtkExtentSplitter * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_extent_splitter_add_extent_source(vtkExtentSplitter* sself, int id, int priority, int x0, int x1, int y0, int y1, int z0, int z1) { sself->AddExtentSource(id, priority, x0, x1, y0, y1, z0, z1); }
extern "C" void vtk_extent_splitter_remove_extent_source(vtkExtentSplitter* sself, int id) { sself->RemoveExtentSource(id); }
extern "C" void vtk_extent_splitter_remove_all_extent_sources(vtkExtentSplitter* sself) { sself->RemoveAllExtentSources(); }
extern "C" void vtk_extent_splitter_add_extent(vtkExtentSplitter* sself, int x0, int x1, int y0, int y1, int z0, int z1) { sself->AddExtent(x0, x1, y0, y1, z0, z1); }
extern "C" int vtk_extent_splitter_compute_sub_extents(vtkExtentSplitter* sself) { return sself->ComputeSubExtents(); }
extern "C" int vtk_extent_splitter_get_number_of_sub_extents(vtkExtentSplitter* sself) { return sself->GetNumberOfSubExtents(); }
extern "C" int vtk_extent_splitter_get_sub_extent_source(vtkExtentSplitter* sself, int index) { return sself->GetSubExtentSource(index); }
extern "C" int vtk_extent_splitter_get_point_mode(vtkExtentSplitter* sself) { return sself->GetPointMode(); }
extern "C" void vtk_extent_splitter_set_point_mode(vtkExtentSplitter* sself, int _arg) { sself->SetPointMode(_arg); }
extern "C" void vtk_extent_splitter_point_mode_on(vtkExtentSplitter* sself) { sself->PointModeOn(); }
extern "C" void vtk_extent_splitter_point_mode_off(vtkExtentSplitter* sself) { sself->PointModeOff(); }
extern "C" vtkExtentTranslator * vtkExtentTranslator_new () {return vtkExtentTranslator :: New () ;}
extern "C" void vtkExtentTranslator_destructor (vtkExtentTranslator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_extent_translator_set_whole_extent(vtkExtentTranslator* sself, int _arg1, int _arg2, int _arg3, int _arg4, int _arg5, int _arg6) { sself->SetWholeExtent(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6); }
extern "C" void vtk_extent_translator_set_extent(vtkExtentTranslator* sself, int _arg1, int _arg2, int _arg3, int _arg4, int _arg5, int _arg6) { sself->SetExtent(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6); }
extern "C" void vtk_extent_translator_set_piece(vtkExtentTranslator* sself, int _arg) { sself->SetPiece(_arg); }
extern "C" int vtk_extent_translator_get_piece(vtkExtentTranslator* sself) { return sself->GetPiece(); }
extern "C" void vtk_extent_translator_set_number_of_pieces(vtkExtentTranslator* sself, int _arg) { sself->SetNumberOfPieces(_arg); }
extern "C" int vtk_extent_translator_get_number_of_pieces(vtkExtentTranslator* sself) { return sself->GetNumberOfPieces(); }
extern "C" void vtk_extent_translator_set_ghost_level(vtkExtentTranslator* sself, int _arg) { sself->SetGhostLevel(_arg); }
extern "C" int vtk_extent_translator_get_ghost_level(vtkExtentTranslator* sself) { return sself->GetGhostLevel(); }
extern "C" int vtk_extent_translator_piece_to_extent(vtkExtentTranslator* sself) { return sself->PieceToExtent(); }
extern "C" int vtk_extent_translator_piece_to_extent_by_points(vtkExtentTranslator* sself) { return sself->PieceToExtentByPoints(); }
extern "C" void vtk_extent_translator_set_split_mode_to_block(vtkExtentTranslator* sself) { sself->SetSplitModeToBlock(); }
extern "C" void vtk_extent_translator_set_split_mode_to_x_slab(vtkExtentTranslator* sself) { sself->SetSplitModeToXSlab(); }
extern "C" void vtk_extent_translator_set_split_mode_to_y_slab(vtkExtentTranslator* sself) { sself->SetSplitModeToYSlab(); }
extern "C" void vtk_extent_translator_set_split_mode_to_z_slab(vtkExtentTranslator* sself) { sself->SetSplitModeToZSlab(); }
extern "C" int vtk_extent_translator_get_split_mode(vtkExtentTranslator* sself) { return sself->GetSplitMode(); }
extern "C" vtkGraphAlgorithm * vtkGraphAlgorithm_new () {return vtkGraphAlgorithm :: New () ;}
extern "C" void vtkGraphAlgorithm_destructor (vtkGraphAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkHierarchicalBoxDataSetAlgorithm * vtkHierarchicalBoxDataSetAlgorithm_new () {return vtkHierarchicalBoxDataSetAlgorithm :: New () ;}
extern "C" void vtkHierarchicalBoxDataSetAlgorithm_destructor (vtkHierarchicalBoxDataSetAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkImageToStructuredGrid * vtkImageToStructuredGrid_new () {return vtkImageToStructuredGrid :: New () ;}
extern "C" void vtkImageToStructuredGrid_destructor (vtkImageToStructuredGrid * sself) {sself -> Delete () ; return ;}
extern "C" vtkImageToStructuredPoints * vtkImageToStructuredPoints_new () {return vtkImageToStructuredPoints :: New () ;}
extern "C" void vtkImageToStructuredPoints_destructor (vtkImageToStructuredPoints * sself) {sself -> Delete () ; return ;}
extern "C" vtkMoleculeAlgorithm * vtkMoleculeAlgorithm_new () {return vtkMoleculeAlgorithm :: New () ;}
extern "C" void vtkMoleculeAlgorithm_destructor (vtkMoleculeAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkMultiBlockDataSetAlgorithm * vtkMultiBlockDataSetAlgorithm_new () {return vtkMultiBlockDataSetAlgorithm :: New () ;}
extern "C" void vtkMultiBlockDataSetAlgorithm_destructor (vtkMultiBlockDataSetAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkMultiTimeStepAlgorithm * vtkMultiTimeStepAlgorithm_new () {return vtkMultiTimeStepAlgorithm :: New () ;}
extern "C" void vtkMultiTimeStepAlgorithm_destructor (vtkMultiTimeStepAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkNonOverlappingAMRAlgorithm * vtkNonOverlappingAMRAlgorithm_new () {return vtkNonOverlappingAMRAlgorithm :: New () ;}
extern "C" void vtkNonOverlappingAMRAlgorithm_destructor (vtkNonOverlappingAMRAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkOverlappingAMRAlgorithm * vtkOverlappingAMRAlgorithm_new () {return vtkOverlappingAMRAlgorithm :: New () ;}
extern "C" void vtkOverlappingAMRAlgorithm_destructor (vtkOverlappingAMRAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkPassInputTypeAlgorithm * vtkPassInputTypeAlgorithm_new () {return vtkPassInputTypeAlgorithm :: New () ;}
extern "C" void vtkPassInputTypeAlgorithm_destructor (vtkPassInputTypeAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkPiecewiseFunctionAlgorithm * vtkPiecewiseFunctionAlgorithm_new () {return vtkPiecewiseFunctionAlgorithm :: New () ;}
extern "C" void vtkPiecewiseFunctionAlgorithm_destructor (vtkPiecewiseFunctionAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkPiecewiseFunctionShiftScale * vtkPiecewiseFunctionShiftScale_new () {return vtkPiecewiseFunctionShiftScale :: New () ;}
extern "C" void vtkPiecewiseFunctionShiftScale_destructor (vtkPiecewiseFunctionShiftScale * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_piecewise_function_shift_scale_set_position_shift(vtkPiecewiseFunctionShiftScale* sself, double _arg) { sself->SetPositionShift(_arg); }
extern "C" void vtk_piecewise_function_shift_scale_set_position_scale(vtkPiecewiseFunctionShiftScale* sself, double _arg) { sself->SetPositionScale(_arg); }
extern "C" void vtk_piecewise_function_shift_scale_set_value_shift(vtkPiecewiseFunctionShiftScale* sself, double _arg) { sself->SetValueShift(_arg); }
extern "C" void vtk_piecewise_function_shift_scale_set_value_scale(vtkPiecewiseFunctionShiftScale* sself, double _arg) { sself->SetValueScale(_arg); }
extern "C" double vtk_piecewise_function_shift_scale_get_position_shift(vtkPiecewiseFunctionShiftScale* sself) { return sself->GetPositionShift(); }
extern "C" double vtk_piecewise_function_shift_scale_get_position_scale(vtkPiecewiseFunctionShiftScale* sself) { return sself->GetPositionScale(); }
extern "C" double vtk_piecewise_function_shift_scale_get_value_shift(vtkPiecewiseFunctionShiftScale* sself) { return sself->GetValueShift(); }
extern "C" double vtk_piecewise_function_shift_scale_get_value_scale(vtkPiecewiseFunctionShiftScale* sself) { return sself->GetValueScale(); }
extern "C" vtkPointSetAlgorithm * vtkPointSetAlgorithm_new () {return vtkPointSetAlgorithm :: New () ;}
extern "C" void vtkPointSetAlgorithm_destructor (vtkPointSetAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkPolyDataAlgorithm * vtkPolyDataAlgorithm_new () {return vtkPolyDataAlgorithm :: New () ;}
extern "C" void vtkPolyDataAlgorithm_destructor (vtkPolyDataAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkProgressObserver * vtkProgressObserver_new () {return vtkProgressObserver :: New () ;}
extern "C" void vtkProgressObserver_destructor (vtkProgressObserver * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_progress_observer_update_progress(vtkProgressObserver* sself, double amount) { sself->UpdateProgress(amount); }
extern "C" double vtk_progress_observer_get_progress(vtkProgressObserver* sself) { return sself->GetProgress(); }
extern "C" vtkReaderExecutive * vtkReaderExecutive_new () {return vtkReaderExecutive :: New () ;}
extern "C" void vtkReaderExecutive_destructor (vtkReaderExecutive * sself) {sself -> Delete () ; return ;}
extern "C" vtkRectilinearGridAlgorithm * vtkRectilinearGridAlgorithm_new () {return vtkRectilinearGridAlgorithm :: New () ;}
extern "C" void vtkRectilinearGridAlgorithm_destructor (vtkRectilinearGridAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkSMPProgressObserver * vtkSMPProgressObserver_new () {return vtkSMPProgressObserver :: New () ;}
extern "C" void vtkSMPProgressObserver_destructor (vtkSMPProgressObserver * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_smp_progress_observer_update_progress(vtkSMPProgressObserver* sself, double progress) { sself->UpdateProgress(progress); }
extern "C" vtkSelectionAlgorithm * vtkSelectionAlgorithm_new () {return vtkSelectionAlgorithm :: New () ;}
extern "C" void vtkSelectionAlgorithm_destructor (vtkSelectionAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkSimpleScalarTree * vtkSimpleScalarTree_new () {return vtkSimpleScalarTree :: New () ;}
extern "C" void vtkSimpleScalarTree_destructor (vtkSimpleScalarTree * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_simple_scalar_tree_set_branching_factor(vtkSimpleScalarTree* sself, int _arg) { sself->SetBranchingFactor(_arg); }
extern "C" int vtk_simple_scalar_tree_get_branching_factor_min_value(vtkSimpleScalarTree* sself) { return sself->GetBranchingFactorMinValue(); }
extern "C" int vtk_simple_scalar_tree_get_branching_factor_max_value(vtkSimpleScalarTree* sself) { return sself->GetBranchingFactorMaxValue(); }
extern "C" int vtk_simple_scalar_tree_get_branching_factor(vtkSimpleScalarTree* sself) { return sself->GetBranchingFactor(); }
extern "C" int vtk_simple_scalar_tree_get_level(vtkSimpleScalarTree* sself) { return sself->GetLevel(); }
extern "C" void vtk_simple_scalar_tree_set_max_level(vtkSimpleScalarTree* sself, int _arg) { sself->SetMaxLevel(_arg); }
extern "C" int vtk_simple_scalar_tree_get_max_level_min_value(vtkSimpleScalarTree* sself) { return sself->GetMaxLevelMinValue(); }
extern "C" int vtk_simple_scalar_tree_get_max_level_max_value(vtkSimpleScalarTree* sself) { return sself->GetMaxLevelMaxValue(); }
extern "C" int vtk_simple_scalar_tree_get_max_level(vtkSimpleScalarTree* sself) { return sself->GetMaxLevel(); }
extern "C" void vtk_simple_scalar_tree_build_tree(vtkSimpleScalarTree* sself) { sself->BuildTree(); }
extern "C" void vtk_simple_scalar_tree_initialize(vtkSimpleScalarTree* sself) { sself->Initialize(); }
extern "C" void vtk_simple_scalar_tree_init_traversal(vtkSimpleScalarTree* sself, double scalarValue) { sself->InitTraversal(scalarValue); }
extern "C" long long vtk_simple_scalar_tree_get_number_of_cell_batches(vtkSimpleScalarTree* sself, double scalarValue) { return sself->GetNumberOfCellBatches(scalarValue); }
extern "C" vtkSpanSpace * vtkSpanSpace_new () {return vtkSpanSpace :: New () ;}
extern "C" void vtkSpanSpace_destructor (vtkSpanSpace * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_span_space_set_scalar_range(vtkSpanSpace* sself, double _arg1, double _arg2) { sself->SetScalarRange(_arg1, _arg2); }
extern "C" void vtk_span_space_set_compute_scalar_range(vtkSpanSpace* sself, int _arg) { sself->SetComputeScalarRange(_arg); }
extern "C" int vtk_span_space_get_compute_scalar_range(vtkSpanSpace* sself) { return sself->GetComputeScalarRange(); }
extern "C" void vtk_span_space_compute_scalar_range_on(vtkSpanSpace* sself) { sself->ComputeScalarRangeOn(); }
extern "C" void vtk_span_space_compute_scalar_range_off(vtkSpanSpace* sself) { sself->ComputeScalarRangeOff(); }
extern "C" void vtk_span_space_set_resolution(vtkSpanSpace* sself, long long _arg) { sself->SetResolution(_arg); }
extern "C" long long vtk_span_space_get_resolution_min_value(vtkSpanSpace* sself) { return sself->GetResolutionMinValue(); }
extern "C" long long vtk_span_space_get_resolution_max_value(vtkSpanSpace* sself) { return sself->GetResolutionMaxValue(); }
extern "C" long long vtk_span_space_get_resolution(vtkSpanSpace* sself) { return sself->GetResolution(); }
extern "C" void vtk_span_space_set_compute_resolution(vtkSpanSpace* sself, int _arg) { sself->SetComputeResolution(_arg); }
extern "C" int vtk_span_space_get_compute_resolution(vtkSpanSpace* sself) { return sself->GetComputeResolution(); }
extern "C" void vtk_span_space_compute_resolution_on(vtkSpanSpace* sself) { sself->ComputeResolutionOn(); }
extern "C" void vtk_span_space_compute_resolution_off(vtkSpanSpace* sself) { sself->ComputeResolutionOff(); }
extern "C" void vtk_span_space_set_number_of_cells_per_bucket(vtkSpanSpace* sself, int _arg) { sself->SetNumberOfCellsPerBucket(_arg); }
extern "C" int vtk_span_space_get_number_of_cells_per_bucket_min_value(vtkSpanSpace* sself) { return sself->GetNumberOfCellsPerBucketMinValue(); }
extern "C" int vtk_span_space_get_number_of_cells_per_bucket_max_value(vtkSpanSpace* sself) { return sself->GetNumberOfCellsPerBucketMaxValue(); }
extern "C" int vtk_span_space_get_number_of_cells_per_bucket(vtkSpanSpace* sself) { return sself->GetNumberOfCellsPerBucket(); }
extern "C" void vtk_span_space_initialize(vtkSpanSpace* sself) { sself->Initialize(); }
extern "C" void vtk_span_space_build_tree(vtkSpanSpace* sself) { sself->BuildTree(); }
extern "C" void vtk_span_space_init_traversal(vtkSpanSpace* sself, double scalarValue) { sself->InitTraversal(scalarValue); }
extern "C" long long vtk_span_space_get_number_of_cell_batches(vtkSpanSpace* sself, double scalarValue) { return sself->GetNumberOfCellBatches(scalarValue); }
extern "C" void vtk_span_space_set_batch_size(vtkSpanSpace* sself, long long _arg) { sself->SetBatchSize(_arg); }
extern "C" long long vtk_span_space_get_batch_size_min_value(vtkSpanSpace* sself) { return sself->GetBatchSizeMinValue(); }
extern "C" long long vtk_span_space_get_batch_size_max_value(vtkSpanSpace* sself) { return sself->GetBatchSizeMaxValue(); }
extern "C" long long vtk_span_space_get_batch_size(vtkSpanSpace* sself) { return sself->GetBatchSize(); }
extern "C" vtkSphereTree * vtkSphereTree_new () {return vtkSphereTree :: New () ;}
extern "C" void vtkSphereTree_destructor (vtkSphereTree * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_sphere_tree_build(vtkSphereTree* sself) { sself->Build(); }
extern "C" void vtk_sphere_tree_set_build_hierarchy(vtkSphereTree* sself, bool _arg) { sself->SetBuildHierarchy(_arg); }
extern "C" bool vtk_sphere_tree_get_build_hierarchy(vtkSphereTree* sself) { return sself->GetBuildHierarchy(); }
extern "C" void vtk_sphere_tree_build_hierarchy_on(vtkSphereTree* sself) { sself->BuildHierarchyOn(); }
extern "C" void vtk_sphere_tree_build_hierarchy_off(vtkSphereTree* sself) { sself->BuildHierarchyOff(); }
extern "C" void vtk_sphere_tree_set_resolution(vtkSphereTree* sself, int _arg) { sself->SetResolution(_arg); }
extern "C" int vtk_sphere_tree_get_resolution_min_value(vtkSphereTree* sself) { return sself->GetResolutionMinValue(); }
extern "C" int vtk_sphere_tree_get_resolution_max_value(vtkSphereTree* sself) { return sself->GetResolutionMaxValue(); }
extern "C" int vtk_sphere_tree_get_resolution(vtkSphereTree* sself) { return sself->GetResolution(); }
extern "C" void vtk_sphere_tree_set_max_level(vtkSphereTree* sself, int _arg) { sself->SetMaxLevel(_arg); }
extern "C" int vtk_sphere_tree_get_max_level_min_value(vtkSphereTree* sself) { return sself->GetMaxLevelMinValue(); }
extern "C" int vtk_sphere_tree_get_max_level_max_value(vtkSphereTree* sself) { return sself->GetMaxLevelMaxValue(); }
extern "C" int vtk_sphere_tree_get_max_level(vtkSphereTree* sself) { return sself->GetMaxLevel(); }
extern "C" int vtk_sphere_tree_get_number_of_levels(vtkSphereTree* sself) { return sself->GetNumberOfLevels(); }
extern "C" vtkStreamingDemandDrivenPipeline * vtkStreamingDemandDrivenPipeline_new () {return vtkStreamingDemandDrivenPipeline :: New () ;}
extern "C" void vtkStreamingDemandDrivenPipeline_destructor (vtkStreamingDemandDrivenPipeline * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_streaming_demand_driven_pipeline_update(vtkStreamingDemandDrivenPipeline* sself) { return sself->Update(); }
extern "C" int vtk_streaming_demand_driven_pipeline_update_whole_extent(vtkStreamingDemandDrivenPipeline* sself) { return sself->UpdateWholeExtent(); }
extern "C" int vtk_streaming_demand_driven_pipeline_propagate_update_extent(vtkStreamingDemandDrivenPipeline* sself, int outputPort) { return sself->PropagateUpdateExtent(outputPort); }
extern "C" int vtk_streaming_demand_driven_pipeline_propagate_time(vtkStreamingDemandDrivenPipeline* sself, int outputPort) { return sself->PropagateTime(outputPort); }
extern "C" int vtk_streaming_demand_driven_pipeline_update_time_dependent_information(vtkStreamingDemandDrivenPipeline* sself, int outputPort) { return sself->UpdateTimeDependentInformation(outputPort); }
extern "C" int vtk_streaming_demand_driven_pipeline_set_request_exact_extent(vtkStreamingDemandDrivenPipeline* sself, int port, int flag) { return sself->SetRequestExactExtent(port, flag); }
extern "C" int vtk_streaming_demand_driven_pipeline_get_request_exact_extent(vtkStreamingDemandDrivenPipeline* sself, int port) { return sself->GetRequestExactExtent(port); }
extern "C" vtkStructuredGridAlgorithm * vtkStructuredGridAlgorithm_new () {return vtkStructuredGridAlgorithm :: New () ;}
extern "C" void vtkStructuredGridAlgorithm_destructor (vtkStructuredGridAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkTableAlgorithm * vtkTableAlgorithm_new () {return vtkTableAlgorithm :: New () ;}
extern "C" void vtkTableAlgorithm_destructor (vtkTableAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkThreadedCompositeDataPipeline * vtkThreadedCompositeDataPipeline_new () {return vtkThreadedCompositeDataPipeline :: New () ;}
extern "C" void vtkThreadedCompositeDataPipeline_destructor (vtkThreadedCompositeDataPipeline * sself) {sself -> Delete () ; return ;}
extern "C" vtkTreeAlgorithm * vtkTreeAlgorithm_new () {return vtkTreeAlgorithm :: New () ;}
extern "C" void vtkTreeAlgorithm_destructor (vtkTreeAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkTrivialConsumer * vtkTrivialConsumer_new () {return vtkTrivialConsumer :: New () ;}
extern "C" void vtkTrivialConsumer_destructor (vtkTrivialConsumer * sself) {sself -> Delete () ; return ;}
extern "C" vtkTrivialProducer * vtkTrivialProducer_new () {return vtkTrivialProducer :: New () ;}
extern "C" void vtkTrivialProducer_destructor (vtkTrivialProducer * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_trivial_producer_get_m_time(vtkTrivialProducer* sself) { return sself->GetMTime(); }
extern "C" void vtk_trivial_producer_set_whole_extent(vtkTrivialProducer* sself, int _arg1, int _arg2, int _arg3, int _arg4, int _arg5, int _arg6) { sself->SetWholeExtent(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6); }
extern "C" vtkUndirectedGraphAlgorithm * vtkUndirectedGraphAlgorithm_new () {return vtkUndirectedGraphAlgorithm :: New () ;}
extern "C" void vtkUndirectedGraphAlgorithm_destructor (vtkUndirectedGraphAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkUniformGridAMRAlgorithm * vtkUniformGridAMRAlgorithm_new () {return vtkUniformGridAMRAlgorithm :: New () ;}
extern "C" void vtkUniformGridAMRAlgorithm_destructor (vtkUniformGridAMRAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkUniformGridPartitioner * vtkUniformGridPartitioner_new () {return vtkUniformGridPartitioner :: New () ;}
extern "C" void vtkUniformGridPartitioner_destructor (vtkUniformGridPartitioner * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_uniform_grid_partitioner_get_number_of_partitions(vtkUniformGridPartitioner* sself) { return sself->GetNumberOfPartitions(); }
extern "C" void vtk_uniform_grid_partitioner_set_number_of_partitions(vtkUniformGridPartitioner* sself, int _arg) { sself->SetNumberOfPartitions(_arg); }
extern "C" int vtk_uniform_grid_partitioner_get_number_of_ghost_layers(vtkUniformGridPartitioner* sself) { return sself->GetNumberOfGhostLayers(); }
extern "C" void vtk_uniform_grid_partitioner_set_number_of_ghost_layers(vtkUniformGridPartitioner* sself, int _arg) { sself->SetNumberOfGhostLayers(_arg); }
extern "C" int vtk_uniform_grid_partitioner_get_duplicate_nodes(vtkUniformGridPartitioner* sself) { return sself->GetDuplicateNodes(); }
extern "C" void vtk_uniform_grid_partitioner_set_duplicate_nodes(vtkUniformGridPartitioner* sself, int _arg) { sself->SetDuplicateNodes(_arg); }
extern "C" void vtk_uniform_grid_partitioner_duplicate_nodes_on(vtkUniformGridPartitioner* sself) { sself->DuplicateNodesOn(); }
extern "C" void vtk_uniform_grid_partitioner_duplicate_nodes_off(vtkUniformGridPartitioner* sself) { sself->DuplicateNodesOff(); }
extern "C" vtkUnstructuredGridAlgorithm * vtkUnstructuredGridAlgorithm_new () {return vtkUnstructuredGridAlgorithm :: New () ;}
extern "C" void vtkUnstructuredGridAlgorithm_destructor (vtkUnstructuredGridAlgorithm * sself) {sself -> Delete () ; return ;}
extern "C" vtkUnstructuredGridBaseAlgorithm * vtkUnstructuredGridBaseAlgorithm_new () {return vtkUnstructuredGridBaseAlgorithm :: New () ;}
extern "C" void vtkUnstructuredGridBaseAlgorithm_destructor (vtkUnstructuredGridBaseAlgorithm * sself) {sself -> Delete () ; return ;}
