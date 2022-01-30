import dlb.fs
import dlb.ex
import dlb_contrib.gcc

class CCompiler(dlb_contrib.gcc.CCompilerGcc):
    DEFINITIONS = {'UNITY_FIXTURE_NO_EXTRAS':1}

# compile and link application written in C
with dlb.ex.Context():
    source_directory = dlb.fs.Path('test/')
    unity_include_directory = dlb.fs.Path('test/unity/')
    output_directory = dlb.fs.Path('test/build/')

    with dlb.di.Cluster('compile'), dlb.ex.Context(max_parallel_redo_count=4):
        compile_results = [
            CCompiler(
                source_files=[p],
                object_files=[output_directory / p.with_appended_suffix('.o')],
                include_search_directories=[source_directory, unity_include_directory],

            ).start()
            for p in source_directory.iterdir(name_filter=r'.+\.c', is_dir=False, recurse_name_filter=lambda n: '.' not in n)
        ]

    with dlb.di.Cluster('link'), dlb.ex.Context():
        object_files = [r.object_files[0] for r in compile_results]
        dlb_contrib.gcc.CLinkerGcc(
            object_and_archive_files=object_files,
            linked_file=output_directory / 'unity_test').start()

dlb.di.inform('finished successfully')
