import dlb.fs
import dlb.ex
import dlb_contrib.gcc

class CCompiler(dlb_contrib.gcc.CCompilerGcc):
    DEFINITIONS = {'UNITY_FIXTURE_NO_EXTRAS':1}

class UnitTest(dlb.ex.Tool):
    EXECUTABLE = 'unity_test'

    async def redo(self, result, context):
        # context.helper['unity_test/'] =  'dist/test/'
        await context.execute_helper(
            self.EXECUTABLE,
            cwd='dist/test/'
        )

# compile and link application written in C
with dlb.ex.Context():
    source_directory = dlb.fs.Path('test/')
    unity_include_directory = dlb.fs.Path('test/unity/')
    build_output_directory = dlb.fs.Path('build/')
    distribution_directory = dlb.fs.Path('dist/test/')

    with dlb.di.Cluster('Compile'), dlb.ex.Context(max_parallel_redo_count=4):
        compile_results = [
            CCompiler(
                source_files=[p],
                object_files=[build_output_directory / p.with_appended_suffix('.o')],
                include_search_directories=[source_directory, unity_include_directory],

            ).start()
            for p in source_directory.iterdir(name_filter=r'.+\.(?:c|cpp)', is_dir=False, recurse_name_filter=lambda n: '.' not in n)
        ]

    with dlb.di.Cluster('Link'), dlb.ex.Context():
        object_files = [r.object_files[0] for r in compile_results]
        dlb_contrib.gcc.CLinkerGcc(
            object_and_archive_files=object_files,
            linked_file=distribution_directory / 'unity_test').start()

    with dlb.di.Cluster('Test'), dlb.ex.Context():
        # TODO: Check how to do define a tool form a generated build-product
        dlb.ex.Context.active.helper['unity_test'] = '/Users/louismayencourt/project/wordclock_upstream/dist/test/unity_test'
        UnitTest().start()

dlb.di.inform('finished successfully')
