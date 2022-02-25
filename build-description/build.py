from distutils.log import ERROR
from logging import CRITICAL, WARNING
import dlb.fs
import dlb.ex
import dlb.di
import dlb.cf
import dlb_contrib.gcc

from typing import List, Union

class CXtensaEsp32Compiler(dlb_contrib.gcc._CompilerGcc):
    EXECUTABLE = 'xtensa-esp32-elf-gcc'
    DIALECT = 'gnu99'
    LANGUAGE = 'c'

    DEFINITIONS = {'ESP_PLATFORM':"-DMBEDTLS_CONFIG_FILE=\"mbedtls/esp_config.h\"", 'HAVE_CONFIG_H':1, 'GCC_NOT_5_2_0':0, 'WITH_POSIX':1, 'F_CPU':'240000000L', 'ARDUINO':'10816', 'ARDUINO_ESP32_DEV':1, 'ARDUINO_ARCH_ESP3':1, 'ARDUINO_BOARD':'\"ESP32_DEV\"" ', 'ARDUINO_VARIANT':'\"esp32\""', 'ESP32':1, 'CORE_DEBUG_LEVEL':'0' }

    def get_extra_compile_arguments(self) -> List[Union[str, dlb.fs.Path, dlb.fs.Path.Native]]:
        compile_arguments = []
        # TODO: Move warnings from this lits to get_warning_compile_arguments()
        compile_arguments.extend(['-Os', '-g3', '-fstack-protector', '-ffunction-sections', '-fdata-sections', '-fstrict-volatile-bitfields', '-mlongcalls', '-nostdlib', '-Wpointer-arith', '-w', '-Wno-maybe-uninitialized', '-Wno-unused-function', '-Wno-unused-but-set-variable', '-Wno-unused-variable', '-Wno-deprecated-declarations', '-Wno-unused-parameter', '-Wno-sign-compare', '-Wno-old-style-declaration', '-MMD', '-c'])
        return compile_arguments

class CppXtensaEsp32Compiler(dlb_contrib.gcc._CompilerGcc):
    EXECUTABLE = 'xtensa-esp32-elf-g++'
    DIALECT = 'gnu++11'
    LANGUAGE = 'c++'

    DEFINITIONS = {'ESP_PLATFORM':"-DMBEDTLS_CONFIG_FILE=\"mbedtls/esp_config.h\"", 'HAVE_CONFIG_H':1, 'GCC_NOT_5_2_0':0, 'WITH_POSIX':1, 'F_CPU':'240000000L', 'ARDUINO':'10816', 'ARDUINO_ESP32_DEV':1, 'ARDUINO_ARCH_ESP3':1, 'ARDUINO_BOARD':'\"ESP32_DEV\"" ', 'ARDUINO_VARIANT':'\"esp32\""', 'ESP32':1, 'CORE_DEBUG_LEVEL':'0' }

    def get_extra_compile_arguments(self) -> List[Union[str, dlb.fs.Path, dlb.fs.Path.Native]]:
        compile_arguments = []
        # TODO: Move warnings from this lits to get_warning_compile_arguments()
        compile_arguments.extend(['-Os', '-g3', '-Wpointer-arith', '-fexceptions', '-fstack-protector', '-ffunction-sections', '-fdata-sections', '-fstrict-volatile-bitfields', '-mlongcalls', '-nostdlib', '-w', '-Wno-error=maybe-uninitialized', '-Wno-error=unused-function', '-Wno-error=unused-but-set-variable', '-Wno-error=unused-variable', '-Wno-error=deprecated-declarations', '-Wno-unused-parameter', '-Wno-unused-but-set-parameter', '-Wno-missing-field-initializers', '-Wno-sign-compare', '-fno-rtti', '-MMD', '-c'])
        return compile_arguments

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
    arduino_esp32_core_source_directory = arduino_esp32_directory / 'cores/esp32/'
    # arduino_esp32_wifi_library = arduino_esp32_directory / 'libraries/Ethernet/'
    # arduino_esp32_wifi_library = arduino_esp32_directory / 'libraries/Ethernet/src/'

    # arduino_esp32_lib_wifi_include_directory = arduino_esp32_directory / 'libraries/WiFi/src/'
    # arduino_esp32_core_include_directory = arduino_esp32_directory / 'cores/esp32/'
    # arduino_esp32_freertos_include_directory = arduino_esp32_directory / 'tools/sdk/esp32/include/freertos/include/esp_additions/freertos/'
    # arduino_esp32_freertos_additions_include_directory = arduino_esp32_directory / 'tools/sdk/esp32/include/freertos/include/esp_additions/'
    # arduino_esp32_sdk_config_include_directory = arduino_esp32_directory / 'tools/sdk/esp32/include/config/'
    # arduino_esp32_sdk_newlib_include_directory = arduino_esp32_directory / 'tools/sdk/esp32/include/newlib/platform_include/'

    arduino_esp32_sdk_directory = arduino_esp32_directory / 'tools/sdk/include/'
    arduino_esp32_sdk_include_directory = []
    # arduino_esp32_sdk_include_directory.append(arduino_esp32_wifi_library)
    # arduino_esp32_sdk_include_directory.append(arduino_esp32_lib_wifi_include_directory)
    # arduino_esp32_sdk_include_directory.append(arduino_esp32_core_include_directory)
    # arduino_esp32_sdk_include_directory.append(arduino_esp32_freertos_include_directory)
    # arduino_esp32_sdk_include_directory.append(arduino_esp32_freertos_additions_include_directory)
    # arduino_esp32_sdk_include_directory.append(arduino_esp32_sdk_config_include_directory)
    # arduino_esp32_sdk_include_directory.append(arduino_esp32_sdk_newlib_include_directory)
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

    test_source_directory = dlb.fs.Path('test/')
    test_spy_sources_directory = dlb.fs.Path('test/spy/')
    unity_include_directory = dlb.fs.Path('test/unity/')

    build_output_directory = dlb.fs.Path('build/')
    distribution_directory = dlb.fs.Path('dist/test/')

    with dlb.di.Cluster('Compile firmware'), dlb.ex.Context():
        dlb.ex.Context.active.helper['xtensa-esp32-elf-g++'] = '/Users/louismayencourt/Library/Arduino15/packages/esp32/tools/xtensa-esp32-elf-gcc/1.22.0-97-gc752ad5-5.2.0/bin/xtensa-esp32-elf-g++'
        dlb.ex.Context.active.helper['xtensa-esp32-elf-gcc'] = '/Users/louismayencourt/Library/Arduino15/packages/esp32/tools/xtensa-esp32-elf-gcc/1.22.0-97-gc752ad5-5.2.0/bin/xtensa-esp32-elf-gcc'

        with dlb.di.Cluster('Compile Arduino C Core files'), dlb.ex.Context(max_parallel_redo_count=8):
            arduino_esp32_core_c_compile_results = [
                        CXtensaEsp32Compiler(
                            source_files=[p],
                            object_files=[build_output_directory / p.with_appended_suffix('.o')],
                            include_search_directories= arduino_esp32_sdk_include_directory,
                        ).start()
                        for p in arduino_esp32_core_source_directory.iterdir(name_filter=r'.+\.c', is_dir=False, recurse_name_filter=lambda n: '.' not in n)
                    ]

        with dlb.di.Cluster('Compile Arduino Cpp Core library'), dlb.ex.Context(max_parallel_redo_count=8):
            arduino_esp32_core_cpp_compile_results = [
                        CppXtensaEsp32Compiler(
                            source_files=[p],
                            object_files=[build_output_directory / p.with_appended_suffix('.o')],
                            include_search_directories=arduino_esp32_sdk_include_directory,
                        ).start()
                        for p in arduino_esp32_core_source_directory.iterdir(name_filter=r'.+\.cpp', is_dir=False, recurse_name_filter=lambda n: '.' not in n)
                    ]

        with dlb.di.Cluster('Compile Firmware hpp'), dlb.ex.Context():
            firmware_hpp_source_directory = dlb.fs.Path('firmware/')
            firmware_hpp_include_directory = [firmware_include_directory]
            firmware_hpp_include_directory.extend(arduino_esp32_sdk_include_directory)
            neopixel_library_directory = dlb.fs.Path('/Users/louismayencourt/Documents/Arduino/libraries/Adafruit_NeoPixel/')
            firmware_hpp_include_directory.append(neopixel_library_directory)

            firmware_hpp_compile_results = [
                CppXtensaEsp32Compiler(
                    source_files=[p],
                    object_files=[build_output_directory / p.with_appended_suffix('.o')],
                    include_search_directories=firmware_hpp_include_directory,
                ).start()
                for p in firmware_hpp_source_directory.iterdir(name_filter=r'.+\.hpp', is_dir=False)
            ]

        firmware_compile_results = [
            CppXtensaEsp32Compiler(
                source_files=[p],
                object_files=[build_output_directory / p.with_appended_suffix('.o')],
                include_search_directories=[firmware_include_directory],
            ).start()
            for p in firmware_source_directory.iterdir(name_filter=r'.+\.cpp', is_dir=False)
        ]

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
            linked_file=distribution_directory / 'unity_test').start().linked_file

    with dlb.di.Cluster('Test'), dlb.ex.Context():
        # TODO: Check how to do define a tool form a generated build-product
        #dlb.ex.Context.active.helper['unity_test'] = '/Users/louismayencourt/project/wordclock_upstream/dist/test/unity_test'
        dlb.ex.Context.active.helper[UnitTest.EXECUTABLE] = test_binary
        UnitTest(
            test_binary=test_binary,
        ).start()


dlb.di.inform('finished successfully')
