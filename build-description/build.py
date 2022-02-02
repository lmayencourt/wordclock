import dlb.fs
import dlb.ex
import dlb_contrib.gcc

from typing import List, Union

class CppCompiler(dlb_contrib.gcc.CplusplusCompilerGcc):
    DEFINITIONS = {'UNITY_FIXTURE_NO_EXTRAS':1}

    def get_extra_compile_arguments(self) -> List[Union[str, dlb.fs.Path, dlb.fs.Path.Native]]:
        compile_arguments = []
        compile_arguments.extend(['-fno-rtti', '-fno-exceptions'])
        return compile_arguments

class UnitTest(dlb.ex.Tool):
    EXECUTABLE = 'unity_test'

    test_binary = dlb.ex.input.RegularFile()

    async def redo(self, result, context):
        await context.execute_helper(
            self.EXECUTABLE,
            cwd='dist/test/'
        )

# compile and link application written in C
with dlb.ex.Context():
    firmware_source_directory = dlb.fs.Path('firmware/src/')
    firmware_include_directory = dlb.fs.Path('firmware/include/')
    test_source_directory = dlb.fs.Path('test/')
    test_spy_sources_directory = dlb.fs.Path('test/spy/')
    unity_include_directory = dlb.fs.Path('test/unity/')
    build_output_directory = dlb.fs.Path('build/')
    distribution_directory = dlb.fs.Path('dist/test/')

    with dlb.di.Cluster('Compile firmware'), dlb.ex.Context():
        firmware_compile_results = [
            CppCompiler(
                source_files=[p],
                object_files=[build_output_directory / p.with_appended_suffix('.o')],
                include_search_directories=[firmware_include_directory],
            ).start()
            for p in firmware_source_directory.iterdir(name_filter=r'.+\.cpp', is_dir=False)
        ]

    with dlb.di.Cluster('Compile tests'), dlb.ex.Context():
        compile_results = [
            CppCompiler(
                source_files=[p],
                object_files=[build_output_directory / p.with_appended_suffix('.o')],
                include_search_directories=[firmware_include_directory,
                                            test_source_directory,
                                            test_spy_sources_directory, 
                                            unity_include_directory],

            ).start()
            for p in test_source_directory.iterdir(name_filter=r'.+\.(?:c|cpp)', is_dir=False, recurse_name_filter=lambda n: '.' not in n)
        ]

    with dlb.di.Cluster('Link tests'), dlb.ex.Context():
        object_files = [r.object_files[0] for r in compile_results]
        object_files+= (r.object_files[0] for r in firmware_compile_results)
        test_binary = dlb_contrib.gcc.CplusplusLinkerGcc(
            object_and_archive_files=object_files,
            linked_file=distribution_directory / 'unity_test').start().linked_file

    with dlb.di.Cluster('Test'), dlb.ex.Context():
        # TODO: Check how to do define a tool form a generated build-product
        #dlb.ex.Context.active.helper['unity_test'] = '/Users/louismayencourt/project/wordclock_upstream/dist/test/unity_test'
        dlb.ex.Context.active.helper[UnitTest.EXECUTABLE] = test_binary
        UnitTest(
            test_binary=test_binary,
        ).start()

dlb.di.inform('finished successfully')
