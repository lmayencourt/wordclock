# SPDX-License-Identifier: LGPL-3.0-or-later
# dlb - a Pythonic build tool
# Copyright (C) 2020 Louis Mayencourt

"""Compile and link languages of the C family with the GNU Compiler Collection (with the help of the
linker from the GNU Binutils)."""

__all__ = [
    'CXtensaEsp32Compiler', 'CppXtensaEsp32Compiler',
    'CppXtensaEsp32Linker',
    'EsptoolElf2Image', 'EsptoolFlash',
    'Esp32Part'
]

import dlb.fs
import dlb.ex
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

class CppXtensaEsp32Linker(dlb_contrib.gcc._LinkerGcc):
    EXECUTABLE = 'xtensa-esp32-elf-gcc'

    LIBRARY_FILENAMES = ('gcc', 'esp_websocket_client', 'wpa2', 'detection', 'esp_https_server', 'wps', 'hal', 'console', 'pe', 'soc', 'sdmmc', 'pthread', 'log', 'esp_http_client', 'json', 'mesh', 'esp32-camera', 'net80211', 'wpa_supplicant', 'c', 'mqtt', 'cxx', 'esp_https_ota', 'ulp', 'efuse', 'pp', 'mdns', 'bt', 'wpa', 'spiffs', 'heap', 'image_util', 'unity', 'rtc', 'mbedtls', 'face_recognition', 'nghttp', 'jsmn', 'openssl', 'core', 'fatfs', 'm', 'protocomm', 'smartconfig', 'xtensa-debug-module', 'dl', 'esp_event', 'esp-tls', 'fd', 'espcoredump', 'esp_http_server', 'fr', 'smartconfig_ack', 'wear_levelling', 'tcp_transport', 'lwip', 'phy', 'vfs', 'coap', 'esp32', 'libsodium', 'bootloader_support', 'driver', 'coexist', 'asio', 'od', 'micro-ecc', 'esp_ringbuf', 'detection_cat_face', 'app_update', 'espnow', 'face_detection', 'app_trace', 'newlib', 'btdm_app', 'wifi_provisioning', 'freertos', 'freemodbus', 'ethernet', 'nvs_flash', 'spi_flash', 'c_nano', 'expat', 'fb_gfx', 'protobuf-c', 'esp_adc_cal', 'tcpip_adapter', 'stdc++')

    def get_all_link_arguments(self) -> List[Union[str, dlb.fs.Path, dlb.fs.Path.Native]]:
        link_arguments = ['-nostdlib']
        link_arguments += self.get_search_link_arguments()
        link_arguments += self.get_extra_link_arguments()
        return link_arguments

    def get_extra_link_arguments(self) -> List[Union[str, dlb.fs.Path, dlb.fs.Path.Native]]:
        link_arguments = ['-Tesp32_out.ld', '-Tesp32.project.ld', '-Tesp32.rom.ld', '-Tesp32.peripherals.ld', '-Tesp32.rom.libgcc.ld', '-Tesp32.rom.spiram_incompatible_fns.ld', '-u esp_app_desc', '-u ld_include_panic_highint_hdl', '-u call_user_start_cpu0', '-Wl,--gc-sections', '-Wl,-static', '-Wl,--undefined=uxTopUsedPriority', '-u __cxa_guard_dummy', '-u __cxx_fatal_exception']
        return link_arguments

    def get_search_link_arguments(self) -> List[Union[str, dlb.fs.Path]]:
            link_arguments = []
            if self.library_search_directories:
                for p in self.library_search_directories:
                    link_arguments.extend(['-L' + p.as_string()])  # looked up for -lxxx
            return link_arguments

    def get_subprogram_link_arguments(self, context) -> List[Union[str, dlb.fs.Path]]:
        return []

    async def redo(self, result, context):
        link_arguments = self.get_all_link_arguments()
        link_arguments += self.get_subprogram_link_arguments(context)

        # link
        with context.temporary() as linked_file:
            link_arguments += [
                '-Wl,--start-group', *result.object_and_archive_files,
            ]
            
            # https://linux.die.net/man/1/ld
            for lib in self.LIBRARY_FILENAMES:
                link_arguments += ['-l' + lib]  # if l is empty: '/usr/bin/ld: cannot find -l:'
            
            link_arguments += ['-Wl,--end-group']

            link_arguments += [
                '-o', linked_file,
                # *result.object_and_archive_files  # note: type detection by suffix of path cannot be disabled
            ]

            await context.execute_helper(self.EXECUTABLE, link_arguments)
            context.replace_output(result.linked_file, linked_file)

class EsptoolElf2Image(dlb.ex.Tool):
    EXECUTABLE = 'esptool'

    input_elf_file = dlb.ex.input.RegularFile()

    output_bin_file = dlb.ex.output.RegularFile(replace_by_same_content=False)

    def get_arguments(self):
        return ['--chip', 'esp32', 'elf2image', '--flash_mode', 'dio', '--flash_freq', '80m', '--flash_size', '4MB']

    async def redo(self, result, context):
        command_arguments = []
        command_arguments = self.get_arguments()
        command_arguments += ['-o', result.output_bin_file, result.input_elf_file]
        await context.execute_helper(self.EXECUTABLE, command_arguments)

class EsptoolFlash(dlb.ex.Tool):
    EXECUTABLE = 'esptool'

    firmware_bin_file = dlb.ex.input.RegularFile()
    partition_bin_file = dlb.ex.input.RegularFile()
    bootloader_bin_file = dlb.ex.input.RegularFile()
    boot_app_bin_file = dlb.ex.input.RegularFile()

    def get_arguments(self):
        return ['--chip', 'esp32', '--port', '/dev/cu.usbserial-0001', '--baud', '921600', '--before', 'default_reset', '--after', 'hard_reset', 'write_flash', '-z', '--flash_mode', 'dio', '--flash_freq', '80m', '--flash_size', 'detect']

    def get_bootloader_arguments(self):
        return ['0xe000', 'tools/partitions/boot_app0.bin', '0x1000', 'tools/sdk/bin/bootloader_qio_80m.bin', '0x10000', 'firmware.ino.bin', '0x8000', 'firmware.ino.partitions.bin']

    async def redo(self, result, context):
        command_arguments = []
        command_arguments = self.get_arguments()
        command_arguments += ['0xe000', result.boot_app_bin_file, '0x1000', result.bootloader_bin_file, '0x10000', result.firmware_bin_file, '0x8000', result.partition_bin_file]
        await context.execute_helper(self.EXECUTABLE, command_arguments)

class Esp32Part(dlb.ex.Tool):
    EXECUTABLE = 'python'

    SCRIPT = '/Users/louismayencourt/Library/Arduino15/packages/esp32/hardware/esp32/1.0.6/tools/gen_esp32part.py'

    csv_file = dlb.ex.input.RegularFile()

    partition_bin_file = dlb.ex.output.RegularFile(replace_by_same_content=False)

    async def redo(self, result, context):
        await context.execute_helper(self.EXECUTABLE, [self.SCRIPT, '-q', result.csv_file, result.partition_bin_file])