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

// Declare exported functions
extern "C" vtkAlgorithm * vtkAlgorithm_new () ;
extern "C" void vtkAlgorithm_destructor (vtkAlgorithm * sself) ;
extern "C" int vtk_algorithm_has_executive(vtkAlgorithm* sself);
extern "C" int vtk_algorithm_get_number_of_input_ports(vtkAlgorithm* sself);
extern "C" int vtk_algorithm_get_number_of_output_ports(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_set_abort_execute(vtkAlgorithm* sself, int _arg);
extern "C" int vtk_algorithm_get_abort_execute(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_abort_execute_on(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_abort_execute_off(vtkAlgorithm* sself);
extern "C" double vtk_algorithm_get_progress(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_set_progress(vtkAlgorithm* sself, double p0);
extern "C" void vtk_algorithm_update_progress(vtkAlgorithm* sself, double amount);
extern "C" void vtk_algorithm_set_progress_shift_scale(vtkAlgorithm* sself, double shift, double scale);
extern "C" double vtk_algorithm_get_progress_shift(vtkAlgorithm* sself);
extern "C" double vtk_algorithm_get_progress_scale(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_set_progress_text(vtkAlgorithm* sself, const char* ptext);
extern "C" unsigned long vtk_algorithm_get_error_code(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_set_input_array_to_process(vtkAlgorithm* sself, int idx, int port, int connection, int fieldAssociation, const char* name);
extern "C" void vtk_algorithm_remove_all_inputs(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_remove_all_input_connections(vtkAlgorithm* sself, int port);
extern "C" int vtk_algorithm_get_number_of_input_connections(vtkAlgorithm* sself, int port);
extern "C" int vtk_algorithm_get_total_number_of_input_connections(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_update(vtkAlgorithm* sself, int port);
extern "C" void vtk_algorithm_update_information(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_update_data_object(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_propagate_update_extent(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_update_whole_extent(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_convert_total_input_to_port_connection(vtkAlgorithm* sself, int ind, int& port, int& conn);
extern "C" void vtk_algorithm_set_release_data_flag(vtkAlgorithm* sself, int p0);
extern "C" int vtk_algorithm_get_release_data_flag(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_release_data_flag_on(vtkAlgorithm* sself);
extern "C" void vtk_algorithm_release_data_flag_off(vtkAlgorithm* sself);
extern "C" int vtk_algorithm_get_update_piece(vtkAlgorithm* sself);
extern "C" int vtk_algorithm_get_update_number_of_pieces(vtkAlgorithm* sself);
extern "C" int vtk_algorithm_get_update_ghost_level(vtkAlgorithm* sself);
extern "C" vtkAlgorithmOutput * vtkAlgorithmOutput_new () ;
extern "C" void vtkAlgorithmOutput_destructor (vtkAlgorithmOutput * sself) ;
extern "C" void vtk_algorithm_output_set_index(vtkAlgorithmOutput* sself, int index);
extern "C" int vtk_algorithm_output_get_index(vtkAlgorithmOutput* sself);
extern "C" vtkAnnotationLayersAlgorithm * vtkAnnotationLayersAlgorithm_new () ;
extern "C" void vtkAnnotationLayersAlgorithm_destructor (vtkAnnotationLayersAlgorithm * sself) ;
extern "C" vtkArrayDataAlgorithm * vtkArrayDataAlgorithm_new () ;
extern "C" void vtkArrayDataAlgorithm_destructor (vtkArrayDataAlgorithm * sself) ;
extern "C" vtkCachedStreamingDemandDrivenPipeline * vtkCachedStreamingDemandDrivenPipeline_new () ;
extern "C" void vtkCachedStreamingDemandDrivenPipeline_destructor (vtkCachedStreamingDemandDrivenPipeline * sself) ;
extern "C" void vtk_cached_streaming_demand_driven_pipeline_set_cache_size(vtkCachedStreamingDemandDrivenPipeline* sself, int size);
extern "C" int vtk_cached_streaming_demand_driven_pipeline_get_cache_size(vtkCachedStreamingDemandDrivenPipeline* sself);
extern "C" vtkCastToConcrete * vtkCastToConcrete_new () ;
extern "C" void vtkCastToConcrete_destructor (vtkCastToConcrete * sself) ;
extern "C" vtkCompositeDataPipeline * vtkCompositeDataPipeline_new () ;
extern "C" void vtkCompositeDataPipeline_destructor (vtkCompositeDataPipeline * sself) ;
extern "C" vtkCompositeDataSetAlgorithm * vtkCompositeDataSetAlgorithm_new () ;
extern "C" void vtkCompositeDataSetAlgorithm_destructor (vtkCompositeDataSetAlgorithm * sself) ;
extern "C" vtkDataObjectAlgorithm * vtkDataObjectAlgorithm_new () ;
extern "C" void vtkDataObjectAlgorithm_destructor (vtkDataObjectAlgorithm * sself) ;
extern "C" vtkDataSetAlgorithm * vtkDataSetAlgorithm_new () ;
extern "C" void vtkDataSetAlgorithm_destructor (vtkDataSetAlgorithm * sself) ;
extern "C" vtkDemandDrivenPipeline * vtkDemandDrivenPipeline_new () ;
extern "C" void vtkDemandDrivenPipeline_destructor (vtkDemandDrivenPipeline * sself) ;
extern "C" unsigned long vtk_demand_driven_pipeline_get_pipeline_m_time(vtkDemandDrivenPipeline* sself);
extern "C" int vtk_demand_driven_pipeline_set_release_data_flag(vtkDemandDrivenPipeline* sself, int port, int n);
extern "C" int vtk_demand_driven_pipeline_get_release_data_flag(vtkDemandDrivenPipeline* sself, int port);
extern "C" int vtk_demand_driven_pipeline_update_pipeline_m_time(vtkDemandDrivenPipeline* sself);
extern "C" int vtk_demand_driven_pipeline_update_data_object(vtkDemandDrivenPipeline* sself);
extern "C" int vtk_demand_driven_pipeline_update_data(vtkDemandDrivenPipeline* sself, int outputPort);
extern "C" vtkDirectedGraphAlgorithm * vtkDirectedGraphAlgorithm_new () ;
extern "C" void vtkDirectedGraphAlgorithm_destructor (vtkDirectedGraphAlgorithm * sself) ;
extern "C" vtkEnsembleSource * vtkEnsembleSource_new () ;
extern "C" void vtkEnsembleSource_destructor (vtkEnsembleSource * sself) ;
extern "C" void vtk_ensemble_source_remove_all_members(vtkEnsembleSource* sself);
extern "C" unsigned int vtk_ensemble_source_get_number_of_members(vtkEnsembleSource* sself);
extern "C" void vtk_ensemble_source_set_current_member(vtkEnsembleSource* sself, unsigned int _arg);
extern "C" unsigned int vtk_ensemble_source_get_current_member(vtkEnsembleSource* sself);
extern "C" vtkExplicitStructuredGridAlgorithm * vtkExplicitStructuredGridAlgorithm_new () ;
extern "C" void vtkExplicitStructuredGridAlgorithm_destructor (vtkExplicitStructuredGridAlgorithm * sself) ;
extern "C" vtkExtentRCBPartitioner * vtkExtentRCBPartitioner_new () ;
extern "C" void vtkExtentRCBPartitioner_destructor (vtkExtentRCBPartitioner * sself) ;
extern "C" void vtk_extent_rcb_partitioner_set_number_of_partitions(vtkExtentRCBPartitioner* sself, const int N);
extern "C" void vtk_extent_rcb_partitioner_set_global_extent(vtkExtentRCBPartitioner* sself, int imin, int imax, int jmin, int jmax, int kmin, int kmax);
extern "C" void vtk_extent_rcb_partitioner_set_duplicate_nodes(vtkExtentRCBPartitioner* sself, int _arg);
extern "C" int vtk_extent_rcb_partitioner_get_duplicate_nodes(vtkExtentRCBPartitioner* sself);
extern "C" void vtk_extent_rcb_partitioner_duplicate_nodes_on(vtkExtentRCBPartitioner* sself);
extern "C" void vtk_extent_rcb_partitioner_duplicate_nodes_off(vtkExtentRCBPartitioner* sself);
extern "C" void vtk_extent_rcb_partitioner_set_number_of_ghost_layers(vtkExtentRCBPartitioner* sself, int _arg);
extern "C" int vtk_extent_rcb_partitioner_get_number_of_ghost_layers(vtkExtentRCBPartitioner* sself);
extern "C" int vtk_extent_rcb_partitioner_get_num_extents(vtkExtentRCBPartitioner* sself);
extern "C" void vtk_extent_rcb_partitioner_partition(vtkExtentRCBPartitioner* sself);
extern "C" vtkExtentSplitter * vtkExtentSplitter_new () ;
extern "C" void vtkExtentSplitter_destructor (vtkExtentSplitter * sself) ;
extern "C" void vtk_extent_splitter_add_extent_source(vtkExtentSplitter* sself, int id, int priority, int x0, int x1, int y0, int y1, int z0, int z1);
extern "C" void vtk_extent_splitter_remove_extent_source(vtkExtentSplitter* sself, int id);
extern "C" void vtk_extent_splitter_remove_all_extent_sources(vtkExtentSplitter* sself);
extern "C" void vtk_extent_splitter_add_extent(vtkExtentSplitter* sself, int x0, int x1, int y0, int y1, int z0, int z1);
extern "C" int vtk_extent_splitter_compute_sub_extents(vtkExtentSplitter* sself);
extern "C" int vtk_extent_splitter_get_number_of_sub_extents(vtkExtentSplitter* sself);
extern "C" int vtk_extent_splitter_get_sub_extent_source(vtkExtentSplitter* sself, int index);
extern "C" int vtk_extent_splitter_get_point_mode(vtkExtentSplitter* sself);
extern "C" void vtk_extent_splitter_set_point_mode(vtkExtentSplitter* sself, int _arg);
extern "C" void vtk_extent_splitter_point_mode_on(vtkExtentSplitter* sself);
extern "C" void vtk_extent_splitter_point_mode_off(vtkExtentSplitter* sself);
extern "C" vtkExtentTranslator * vtkExtentTranslator_new () ;
extern "C" void vtkExtentTranslator_destructor (vtkExtentTranslator * sself) ;
extern "C" void vtk_extent_translator_set_whole_extent(vtkExtentTranslator* sself, int _arg1, int _arg2, int _arg3, int _arg4, int _arg5, int _arg6);
extern "C" void vtk_extent_translator_set_extent(vtkExtentTranslator* sself, int _arg1, int _arg2, int _arg3, int _arg4, int _arg5, int _arg6);
extern "C" void vtk_extent_translator_set_piece(vtkExtentTranslator* sself, int _arg);
extern "C" int vtk_extent_translator_get_piece(vtkExtentTranslator* sself);
extern "C" void vtk_extent_translator_set_number_of_pieces(vtkExtentTranslator* sself, int _arg);
extern "C" int vtk_extent_translator_get_number_of_pieces(vtkExtentTranslator* sself);
extern "C" void vtk_extent_translator_set_ghost_level(vtkExtentTranslator* sself, int _arg);
extern "C" int vtk_extent_translator_get_ghost_level(vtkExtentTranslator* sself);
extern "C" int vtk_extent_translator_piece_to_extent(vtkExtentTranslator* sself);
extern "C" int vtk_extent_translator_piece_to_extent_by_points(vtkExtentTranslator* sself);
extern "C" void vtk_extent_translator_set_split_mode_to_block(vtkExtentTranslator* sself);
extern "C" void vtk_extent_translator_set_split_mode_to_x_slab(vtkExtentTranslator* sself);
extern "C" void vtk_extent_translator_set_split_mode_to_y_slab(vtkExtentTranslator* sself);
extern "C" void vtk_extent_translator_set_split_mode_to_z_slab(vtkExtentTranslator* sself);
extern "C" int vtk_extent_translator_get_split_mode(vtkExtentTranslator* sself);
extern "C" vtkGraphAlgorithm * vtkGraphAlgorithm_new () ;
extern "C" void vtkGraphAlgorithm_destructor (vtkGraphAlgorithm * sself) ;
extern "C" vtkHierarchicalBoxDataSetAlgorithm * vtkHierarchicalBoxDataSetAlgorithm_new () ;
extern "C" void vtkHierarchicalBoxDataSetAlgorithm_destructor (vtkHierarchicalBoxDataSetAlgorithm * sself) ;
extern "C" vtkImageToStructuredGrid * vtkImageToStructuredGrid_new () ;
extern "C" void vtkImageToStructuredGrid_destructor (vtkImageToStructuredGrid * sself) ;
extern "C" vtkImageToStructuredPoints * vtkImageToStructuredPoints_new () ;
extern "C" void vtkImageToStructuredPoints_destructor (vtkImageToStructuredPoints * sself) ;
extern "C" vtkMoleculeAlgorithm * vtkMoleculeAlgorithm_new () ;
extern "C" void vtkMoleculeAlgorithm_destructor (vtkMoleculeAlgorithm * sself) ;
extern "C" vtkMultiBlockDataSetAlgorithm * vtkMultiBlockDataSetAlgorithm_new () ;
extern "C" void vtkMultiBlockDataSetAlgorithm_destructor (vtkMultiBlockDataSetAlgorithm * sself) ;
extern "C" vtkMultiTimeStepAlgorithm * vtkMultiTimeStepAlgorithm_new () ;
extern "C" void vtkMultiTimeStepAlgorithm_destructor (vtkMultiTimeStepAlgorithm * sself) ;
extern "C" vtkNonOverlappingAMRAlgorithm * vtkNonOverlappingAMRAlgorithm_new () ;
extern "C" void vtkNonOverlappingAMRAlgorithm_destructor (vtkNonOverlappingAMRAlgorithm * sself) ;
extern "C" vtkOverlappingAMRAlgorithm * vtkOverlappingAMRAlgorithm_new () ;
extern "C" void vtkOverlappingAMRAlgorithm_destructor (vtkOverlappingAMRAlgorithm * sself) ;
extern "C" vtkPassInputTypeAlgorithm * vtkPassInputTypeAlgorithm_new () ;
extern "C" void vtkPassInputTypeAlgorithm_destructor (vtkPassInputTypeAlgorithm * sself) ;
extern "C" vtkPiecewiseFunctionAlgorithm * vtkPiecewiseFunctionAlgorithm_new () ;
extern "C" void vtkPiecewiseFunctionAlgorithm_destructor (vtkPiecewiseFunctionAlgorithm * sself) ;
extern "C" vtkPiecewiseFunctionShiftScale * vtkPiecewiseFunctionShiftScale_new () ;
extern "C" void vtkPiecewiseFunctionShiftScale_destructor (vtkPiecewiseFunctionShiftScale * sself) ;
extern "C" void vtk_piecewise_function_shift_scale_set_position_shift(vtkPiecewiseFunctionShiftScale* sself, double _arg);
extern "C" void vtk_piecewise_function_shift_scale_set_position_scale(vtkPiecewiseFunctionShiftScale* sself, double _arg);
extern "C" void vtk_piecewise_function_shift_scale_set_value_shift(vtkPiecewiseFunctionShiftScale* sself, double _arg);
extern "C" void vtk_piecewise_function_shift_scale_set_value_scale(vtkPiecewiseFunctionShiftScale* sself, double _arg);
extern "C" double vtk_piecewise_function_shift_scale_get_position_shift(vtkPiecewiseFunctionShiftScale* sself);
extern "C" double vtk_piecewise_function_shift_scale_get_position_scale(vtkPiecewiseFunctionShiftScale* sself);
extern "C" double vtk_piecewise_function_shift_scale_get_value_shift(vtkPiecewiseFunctionShiftScale* sself);
extern "C" double vtk_piecewise_function_shift_scale_get_value_scale(vtkPiecewiseFunctionShiftScale* sself);
extern "C" vtkPointSetAlgorithm * vtkPointSetAlgorithm_new () ;
extern "C" void vtkPointSetAlgorithm_destructor (vtkPointSetAlgorithm * sself) ;
extern "C" vtkPolyDataAlgorithm * vtkPolyDataAlgorithm_new () ;
extern "C" void vtkPolyDataAlgorithm_destructor (vtkPolyDataAlgorithm * sself) ;
extern "C" vtkProgressObserver * vtkProgressObserver_new () ;
extern "C" void vtkProgressObserver_destructor (vtkProgressObserver * sself) ;
extern "C" void vtk_progress_observer_update_progress(vtkProgressObserver* sself, double amount);
extern "C" double vtk_progress_observer_get_progress(vtkProgressObserver* sself);
extern "C" vtkReaderExecutive * vtkReaderExecutive_new () ;
extern "C" void vtkReaderExecutive_destructor (vtkReaderExecutive * sself) ;
extern "C" vtkRectilinearGridAlgorithm * vtkRectilinearGridAlgorithm_new () ;
extern "C" void vtkRectilinearGridAlgorithm_destructor (vtkRectilinearGridAlgorithm * sself) ;
extern "C" vtkSMPProgressObserver * vtkSMPProgressObserver_new () ;
extern "C" void vtkSMPProgressObserver_destructor (vtkSMPProgressObserver * sself) ;
extern "C" void vtk_smp_progress_observer_update_progress(vtkSMPProgressObserver* sself, double progress);
extern "C" vtkSelectionAlgorithm * vtkSelectionAlgorithm_new () ;
extern "C" void vtkSelectionAlgorithm_destructor (vtkSelectionAlgorithm * sself) ;
extern "C" vtkSimpleScalarTree * vtkSimpleScalarTree_new () ;
extern "C" void vtkSimpleScalarTree_destructor (vtkSimpleScalarTree * sself) ;
extern "C" void vtk_simple_scalar_tree_set_branching_factor(vtkSimpleScalarTree* sself, int _arg);
extern "C" int vtk_simple_scalar_tree_get_branching_factor_min_value(vtkSimpleScalarTree* sself);
extern "C" int vtk_simple_scalar_tree_get_branching_factor_max_value(vtkSimpleScalarTree* sself);
extern "C" int vtk_simple_scalar_tree_get_branching_factor(vtkSimpleScalarTree* sself);
extern "C" int vtk_simple_scalar_tree_get_level(vtkSimpleScalarTree* sself);
extern "C" void vtk_simple_scalar_tree_set_max_level(vtkSimpleScalarTree* sself, int _arg);
extern "C" int vtk_simple_scalar_tree_get_max_level_min_value(vtkSimpleScalarTree* sself);
extern "C" int vtk_simple_scalar_tree_get_max_level_max_value(vtkSimpleScalarTree* sself);
extern "C" int vtk_simple_scalar_tree_get_max_level(vtkSimpleScalarTree* sself);
extern "C" void vtk_simple_scalar_tree_build_tree(vtkSimpleScalarTree* sself);
extern "C" void vtk_simple_scalar_tree_initialize(vtkSimpleScalarTree* sself);
extern "C" void vtk_simple_scalar_tree_init_traversal(vtkSimpleScalarTree* sself, double scalarValue);
extern "C" long long vtk_simple_scalar_tree_get_number_of_cell_batches(vtkSimpleScalarTree* sself, double scalarValue);
extern "C" vtkSpanSpace * vtkSpanSpace_new () ;
extern "C" void vtkSpanSpace_destructor (vtkSpanSpace * sself) ;
extern "C" void vtk_span_space_set_scalar_range(vtkSpanSpace* sself, double _arg1, double _arg2);
extern "C" void vtk_span_space_set_compute_scalar_range(vtkSpanSpace* sself, int _arg);
extern "C" int vtk_span_space_get_compute_scalar_range(vtkSpanSpace* sself);
extern "C" void vtk_span_space_compute_scalar_range_on(vtkSpanSpace* sself);
extern "C" void vtk_span_space_compute_scalar_range_off(vtkSpanSpace* sself);
extern "C" void vtk_span_space_set_resolution(vtkSpanSpace* sself, long long _arg);
extern "C" long long vtk_span_space_get_resolution_min_value(vtkSpanSpace* sself);
extern "C" long long vtk_span_space_get_resolution_max_value(vtkSpanSpace* sself);
extern "C" long long vtk_span_space_get_resolution(vtkSpanSpace* sself);
extern "C" void vtk_span_space_set_compute_resolution(vtkSpanSpace* sself, int _arg);
extern "C" int vtk_span_space_get_compute_resolution(vtkSpanSpace* sself);
extern "C" void vtk_span_space_compute_resolution_on(vtkSpanSpace* sself);
extern "C" void vtk_span_space_compute_resolution_off(vtkSpanSpace* sself);
extern "C" void vtk_span_space_set_number_of_cells_per_bucket(vtkSpanSpace* sself, int _arg);
extern "C" int vtk_span_space_get_number_of_cells_per_bucket_min_value(vtkSpanSpace* sself);
extern "C" int vtk_span_space_get_number_of_cells_per_bucket_max_value(vtkSpanSpace* sself);
extern "C" int vtk_span_space_get_number_of_cells_per_bucket(vtkSpanSpace* sself);
extern "C" void vtk_span_space_initialize(vtkSpanSpace* sself);
extern "C" void vtk_span_space_build_tree(vtkSpanSpace* sself);
extern "C" void vtk_span_space_init_traversal(vtkSpanSpace* sself, double scalarValue);
extern "C" long long vtk_span_space_get_number_of_cell_batches(vtkSpanSpace* sself, double scalarValue);
extern "C" void vtk_span_space_set_batch_size(vtkSpanSpace* sself, long long _arg);
extern "C" long long vtk_span_space_get_batch_size_min_value(vtkSpanSpace* sself);
extern "C" long long vtk_span_space_get_batch_size_max_value(vtkSpanSpace* sself);
extern "C" long long vtk_span_space_get_batch_size(vtkSpanSpace* sself);
extern "C" vtkSphereTree * vtkSphereTree_new () ;
extern "C" void vtkSphereTree_destructor (vtkSphereTree * sself) ;
extern "C" void vtk_sphere_tree_build(vtkSphereTree* sself);
extern "C" void vtk_sphere_tree_set_build_hierarchy(vtkSphereTree* sself, bool _arg);
extern "C" bool vtk_sphere_tree_get_build_hierarchy(vtkSphereTree* sself);
extern "C" void vtk_sphere_tree_build_hierarchy_on(vtkSphereTree* sself);
extern "C" void vtk_sphere_tree_build_hierarchy_off(vtkSphereTree* sself);
extern "C" void vtk_sphere_tree_set_resolution(vtkSphereTree* sself, int _arg);
extern "C" int vtk_sphere_tree_get_resolution_min_value(vtkSphereTree* sself);
extern "C" int vtk_sphere_tree_get_resolution_max_value(vtkSphereTree* sself);
extern "C" int vtk_sphere_tree_get_resolution(vtkSphereTree* sself);
extern "C" void vtk_sphere_tree_set_max_level(vtkSphereTree* sself, int _arg);
extern "C" int vtk_sphere_tree_get_max_level_min_value(vtkSphereTree* sself);
extern "C" int vtk_sphere_tree_get_max_level_max_value(vtkSphereTree* sself);
extern "C" int vtk_sphere_tree_get_max_level(vtkSphereTree* sself);
extern "C" int vtk_sphere_tree_get_number_of_levels(vtkSphereTree* sself);
extern "C" vtkStreamingDemandDrivenPipeline * vtkStreamingDemandDrivenPipeline_new () ;
extern "C" void vtkStreamingDemandDrivenPipeline_destructor (vtkStreamingDemandDrivenPipeline * sself) ;
extern "C" int vtk_streaming_demand_driven_pipeline_update(vtkStreamingDemandDrivenPipeline* sself);
extern "C" int vtk_streaming_demand_driven_pipeline_update_whole_extent(vtkStreamingDemandDrivenPipeline* sself);
extern "C" int vtk_streaming_demand_driven_pipeline_propagate_update_extent(vtkStreamingDemandDrivenPipeline* sself, int outputPort);
extern "C" int vtk_streaming_demand_driven_pipeline_propagate_time(vtkStreamingDemandDrivenPipeline* sself, int outputPort);
extern "C" int vtk_streaming_demand_driven_pipeline_update_time_dependent_information(vtkStreamingDemandDrivenPipeline* sself, int outputPort);
extern "C" int vtk_streaming_demand_driven_pipeline_set_request_exact_extent(vtkStreamingDemandDrivenPipeline* sself, int port, int flag);
extern "C" int vtk_streaming_demand_driven_pipeline_get_request_exact_extent(vtkStreamingDemandDrivenPipeline* sself, int port);
extern "C" vtkStructuredGridAlgorithm * vtkStructuredGridAlgorithm_new () ;
extern "C" void vtkStructuredGridAlgorithm_destructor (vtkStructuredGridAlgorithm * sself) ;
extern "C" vtkTableAlgorithm * vtkTableAlgorithm_new () ;
extern "C" void vtkTableAlgorithm_destructor (vtkTableAlgorithm * sself) ;
extern "C" vtkThreadedCompositeDataPipeline * vtkThreadedCompositeDataPipeline_new () ;
extern "C" void vtkThreadedCompositeDataPipeline_destructor (vtkThreadedCompositeDataPipeline * sself) ;
extern "C" vtkTreeAlgorithm * vtkTreeAlgorithm_new () ;
extern "C" void vtkTreeAlgorithm_destructor (vtkTreeAlgorithm * sself) ;
extern "C" vtkTrivialConsumer * vtkTrivialConsumer_new () ;
extern "C" void vtkTrivialConsumer_destructor (vtkTrivialConsumer * sself) ;
extern "C" vtkTrivialProducer * vtkTrivialProducer_new () ;
extern "C" void vtkTrivialProducer_destructor (vtkTrivialProducer * sself) ;
extern "C" unsigned long vtk_trivial_producer_get_m_time(vtkTrivialProducer* sself);
extern "C" void vtk_trivial_producer_set_whole_extent(vtkTrivialProducer* sself, int _arg1, int _arg2, int _arg3, int _arg4, int _arg5, int _arg6);
extern "C" vtkUndirectedGraphAlgorithm * vtkUndirectedGraphAlgorithm_new () ;
extern "C" void vtkUndirectedGraphAlgorithm_destructor (vtkUndirectedGraphAlgorithm * sself) ;
extern "C" vtkUniformGridAMRAlgorithm * vtkUniformGridAMRAlgorithm_new () ;
extern "C" void vtkUniformGridAMRAlgorithm_destructor (vtkUniformGridAMRAlgorithm * sself) ;
extern "C" vtkUniformGridPartitioner * vtkUniformGridPartitioner_new () ;
extern "C" void vtkUniformGridPartitioner_destructor (vtkUniformGridPartitioner * sself) ;
extern "C" int vtk_uniform_grid_partitioner_get_number_of_partitions(vtkUniformGridPartitioner* sself);
extern "C" void vtk_uniform_grid_partitioner_set_number_of_partitions(vtkUniformGridPartitioner* sself, int _arg);
extern "C" int vtk_uniform_grid_partitioner_get_number_of_ghost_layers(vtkUniformGridPartitioner* sself);
extern "C" void vtk_uniform_grid_partitioner_set_number_of_ghost_layers(vtkUniformGridPartitioner* sself, int _arg);
extern "C" int vtk_uniform_grid_partitioner_get_duplicate_nodes(vtkUniformGridPartitioner* sself);
extern "C" void vtk_uniform_grid_partitioner_set_duplicate_nodes(vtkUniformGridPartitioner* sself, int _arg);
extern "C" void vtk_uniform_grid_partitioner_duplicate_nodes_on(vtkUniformGridPartitioner* sself);
extern "C" void vtk_uniform_grid_partitioner_duplicate_nodes_off(vtkUniformGridPartitioner* sself);
extern "C" vtkUnstructuredGridAlgorithm * vtkUnstructuredGridAlgorithm_new () ;
extern "C" void vtkUnstructuredGridAlgorithm_destructor (vtkUnstructuredGridAlgorithm * sself) ;
extern "C" vtkUnstructuredGridBaseAlgorithm * vtkUnstructuredGridBaseAlgorithm_new () ;
extern "C" void vtkUnstructuredGridBaseAlgorithm_destructor (vtkUnstructuredGridBaseAlgorithm * sself) ;
