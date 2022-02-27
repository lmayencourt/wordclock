from ast import arguments
from distutils.log import ERROR
from logging import CRITICAL, WARNING
import dlb.fs
import dlb.ex
import dlb.di
import dlb.cf
import dlb_contrib.gcc

from xtensa_esp32_gcc import *

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
        try:
            await context.execute_helper(
                self.EXECUTABLE,
                cwd='dist/test/'
            )
        except:
            dlb.di.inform("unit-test execution failed", level=CRITICAL)

# compile and link application written in C
with dlb.ex.Context():
    firmware_source_directory = dlb.fs.Path('firmware/src/')
    firmware_include_directory = dlb.fs.Path('firmware/include/')

    arduino_esp32_directory = dlb.fs.Path('external_dependencies/arduino-esp32/')
    arduino_esp32_absolute_directory = dlb.ex.Context.active.root_path / arduino_esp32_directory
    arduino_esp32_core_source_directory = arduino_esp32_directory / 'cores/esp32/'

    arduino_esp32_sdk_directory = arduino_esp32_directory / 'tools/sdk/include/'
    arduino_esp32_sdk_include_directory = []
    # for path in arduino_esp32_sdk_directory.iterdir(name_filter=r'.*', is_dir=True, recurse_name_filter=lambda n: '.' not in n):
    #     # print(path.native)
    #     if path.parts[-1] == 'include':
    #         print(f'add {path.native} to include dir')
    #         arduino_esp32_sdk_include_directory.append(path)

    arduino_esp32_sdk_include_list = [ 'config/', 'app_trace/', 'app_update/', 'asio/', 'bootloader_support/', 'bt/', 'coap/', 'console/', 'driver/', 'efuse/', 'esp-tls/', 'esp32/', 'esp_adc_cal/', 'esp_event/', 'esp_http_client/', 'esp_http_server/', 'esp_https_ota/', 'esp_https_server/', 'esp_ringbuf/', 'esp_websocket_client/', 'espcoredump/', 'ethernet/', 'expat/', 'fatfs/', 'freemodbus/', 'freertos/', 'heap/', 'idf_test/', 'jsmn/', 'json/', 'libsodium/', 'log/', 'lwip/', 'mbedtls/', 'mdns/', 'micro-ecc/', 'mqtt/', 'newlib/', 'nghttp/', 'nvs_flash/', 'openssl/', 'protobuf-c/', 'protocomm/', 'pthread/', 'sdmmc/', 'smartconfig_ack/', 'soc/', 'spi_flash/', 'spiffs/', 'tcp_transport/', 'tcpip_adapter/', 'ulp/', 'unity/', 'vfs/', 'wear_levelling/', 'wifi_provisioning/', 'wpa_supplicant/', 'xtensa-debug-module/', 'esp32-camera/', 'esp-face/', 'fb_gfx/']
    for dir in arduino_esp32_sdk_include_list:
        arduino_esp32_sdk_include_directory.append(arduino_esp32_sdk_directory / dir)

    arduino_esp32_core_include_list = [ 'cores/esp32/', 'variants/esp32/', 'libraries/WiFi/src/', 'libraries/SPIFFS/src/', 'libraries/FS/src/', 'libraries/HTTPClient/src/', 'libraries/WiFiClientSecure/src/', 'libraries/HTTPUpdate/src/', 'libraries/Update/src/']
    for dir in arduino_esp32_core_include_list:
        arduino_esp32_sdk_include_directory.append(arduino_esp32_directory / dir)

    adafruit_neopixel_source_directory = dlb.fs.Path('external_dependencies/Adafruit_NeoPixel/')

    test_source_directory = dlb.fs.Path('test/')
    test_spy_sources_directory = dlb.fs.Path('test/spy/')
    unity_include_directory = dlb.fs.Path('test/unity/')

    build_output_directory = dlb.fs.Path('build/')
    distribution_directory = dlb.fs.Path('dist/')

    parallel_build_redo = 16

    with dlb.di.Cluster('Compile firmware'), dlb.ex.Context():
        dlb.ex.Context.active.helper['xtensa-esp32-elf-g++'] = '/Users/louismayencourt/Library/Arduino15/packages/esp32/tools/xtensa-esp32-elf-gcc/1.22.0-97-gc752ad5-5.2.0/bin/xtensa-esp32-elf-g++'
        dlb.ex.Context.active.helper['xtensa-esp32-elf-gcc'] = '/Users/louismayencourt/Library/Arduino15/packages/esp32/tools/xtensa-esp32-elf-gcc/1.22.0-97-gc752ad5-5.2.0/bin/xtensa-esp32-elf-gcc'

        with dlb.di.Cluster('Compile Arduino C Core files'), dlb.ex.Context(max_parallel_redo_count=parallel_build_redo):
            arduino_esp32_core_c_compile_results = [
                    CXtensaEsp32Compiler(
                        source_files=[p],
                        object_files=[build_output_directory / p.with_appended_suffix('.o')],
                        include_search_directories= arduino_esp32_sdk_include_directory,
                    ).start()
                    for p in arduino_esp32_core_source_directory.iterdir(name_filter=r'.+\.c', is_dir=False, recurse_name_filter=lambda n: '.' not in n)
                ]

        with dlb.di.Cluster('Compile Arduino Cpp Core library'), dlb.ex.Context(max_parallel_redo_count=parallel_build_redo):
            arduino_esp32_core_cpp_compile_results = [
                    CppXtensaEsp32Compiler(
                        source_files=[p],
                        object_files=[build_output_directory / p.with_appended_suffix('.o')],
                        include_search_directories=arduino_esp32_sdk_include_directory,
                    ).start()
                    for p in arduino_esp32_core_source_directory.iterdir(name_filter=r'.+\.cpp', is_dir=False, recurse_name_filter=lambda n: '.' not in n)
                ]

        with dlb.di.Cluster('Compile external libraries'), dlb.ex.Context():
            adafruit_neopixel_compile_results = [
                    CppXtensaEsp32Compiler(
                        source_files=[p],
                        object_files=[build_output_directory / p.with_appended_suffix('.o')],
                        include_search_directories=arduino_esp32_sdk_include_directory,
                    ).start()
                    for p in adafruit_neopixel_source_directory.iterdir(name_filter=r'.+\.cpp', is_dir=False)
                ]

        with dlb.di.Cluster('Compile Firmware hpp'), dlb.ex.Context(max_parallel_redo_count=parallel_build_redo):
            firmware_hpp_source_directory = dlb.fs.Path('src/')
            firmware_hpp_include_directory = [firmware_include_directory]
            firmware_hpp_include_directory.extend(arduino_esp32_sdk_include_directory)
            external_libraries_directories = [dlb.fs.Path('/Users/louismayencourt/Documents/Arduino/libraries/Adafruit_NeoPixel/'),
                                                dlb.fs.Path('/Users/louismayencourt/Documents/Arduino/libraries/ESPAsyncWebServer-master/src/'),
                                                dlb.fs.Path('/Users/louismayencourt/Documents/Arduino/libraries/AsyncTCP-master/src/')]
            firmware_hpp_include_directory.extend(external_libraries_directories)

            firmware_hpp_compile_results = [
                CppXtensaEsp32Compiler(
                    source_files=[p],
                    object_files=[build_output_directory / p.with_appended_suffix('.o')],
                    include_search_directories=firmware_hpp_include_directory,
                ).start()
                for p in firmware_hpp_source_directory.iterdir(name_filter=r'.+\.(hpp|cpp)', is_dir=False)
            ]

        with dlb.di.Cluster('Link Firmware'), dlb.ex.Context():
            dlb.cf.level.helper_execution = dlb.di.INFO
            object_files = [r.object_files[0] for r in arduino_esp32_core_c_compile_results]
            object_files += [r.object_files[0] for r in arduino_esp32_core_cpp_compile_results]
            object_files += [r.object_files[0] for r in adafruit_neopixel_compile_results]
            object_files += (r.object_files[0] for r in firmware_hpp_compile_results)
            firmware_elf_file = CppXtensaEsp32Linker(
                object_and_archive_files=object_files,
                library_search_directories=[arduino_esp32_directory / 'tools/sdk/lib/',
                                            arduino_esp32_directory / 'tools/sdk/ld/'],
                linked_file=distribution_directory / 'firmware'
                ).start().linked_file
            dlb.cf.level.helper_execution = dlb.di.WARNING

            dlb.ex.Context.active.helper[EsptoolElf2Image.EXECUTABLE] = '/Users/louismayencourt/Library/Arduino15/packages/esp32/tools/esptool_py/3.0.0/esptool'
            firmware_bin_file = [
                EsptoolElf2Image(
                    input_elf_file=firmware_elf_file,
                    output_bin_file=distribution_directory / 'firmware.bin',
                ).start()
            ]

            partition_bin_file = [
                Esp32Part(
                    csv_file=arduino_esp32_directory / 'tools/partitions/default.csv',
                    partition_bin_file=distribution_directory / 'firmware.partitions.bin',
                ).start()
            ]

            EsptoolFlash(
                firmware_bin_file=distribution_directory / 'firmware.bin',
                partition_bin_file=distribution_directory / 'firmware.partitions.bin',
                bootloader_bin_file=arduino_esp32_directory / 'tools/sdk/bin/bootloader_qio_80m.bin',
                boot_app_bin_file= arduino_esp32_directory / 'tools/partitions/boot_app0.bin',
            ).start()

    compile_test = False
    if compile_test:
        with dlb.di.Cluster('Compile tests'), dlb.ex.Context():
            firmware_compile_results = [
                CppCompiler(
                    source_files=[p],
                    object_files=[build_output_directory / p.with_appended_suffix('.o')],
                    include_search_directories=[firmware_include_directory],
                ).start()
                for p in firmware_source_directory.iterdir(name_filter=r'.+\.cpp', is_dir=False)
            ]

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
                linked_file=distribution_directory / 'test/unity_test').start().linked_file

        with dlb.di.Cluster('Test'), dlb.ex.Context():
            # TODO: Check how to do define a tool form a generated build-product
            #dlb.ex.Context.active.helper['unity_test'] = '/Users/louismayencourt/project/wordclock_upstream/dist/test/unity_test'
            dlb.ex.Context.active.helper[UnitTest.EXECUTABLE] = test_binary
            UnitTest(
                test_binary=test_binary,
            ).start()


dlb.di.inform('finished successfully')
