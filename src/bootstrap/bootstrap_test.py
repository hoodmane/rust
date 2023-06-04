"""Bootstrap tests

Run these with `x test bootstrap`, or `python -m unittest bootstrap_test.py`."""

from __future__ import absolute_import, division, print_function
import os
import doctest
import unittest
import tempfile
import hashlib
import sys

from shutil import rmtree

import bootstrap
import configure


class VerifyTestCase(unittest.TestCase):
    """Test Case for verify"""
    def setUp(self):
        self.container = tempfile.mkdtemp()
        self.src = os.path.join(self.container, "src.txt")
        self.bad_src = os.path.join(self.container, "bad.txt")
        content = "Hello world"

        self.expected = hashlib.sha256(content.encode("utf-8")).hexdigest()

        with open(self.src, "w") as src:
            src.write(content)
        with open(self.bad_src, "w") as bad:
            bad.write("Hello!")

    def tearDown(self):
        rmtree(self.container)

    def test_valid_file(self):
        """Check if the sha256 sum of the given file is valid"""
        self.assertTrue(bootstrap.verify(self.src, self.expected, False))

    def test_invalid_file(self):
        """Should verify that the file is invalid"""
        self.assertFalse(bootstrap.verify(self.bad_src, self.expected, False))


class ProgramOutOfDate(unittest.TestCase):
    """Test if a program is out of date"""
    def setUp(self):
        self.container = tempfile.mkdtemp()
        os.mkdir(os.path.join(self.container, "stage0"))
        self.build = bootstrap.RustBuild()
        self.build.date = "2017-06-15"
        self.build.build_dir = self.container
        self.rustc_stamp_path = os.path.join(self.container, "stage0",
                                             ".rustc-stamp")
        self.key = self.build.date + str(None)

    def tearDown(self):
        rmtree(self.container)

    def test_stamp_path_does_not_exist(self):
        """Return True when the stamp file does not exist"""
        if os.path.exists(self.rustc_stamp_path):
            os.unlink(self.rustc_stamp_path)
        self.assertTrue(self.build.program_out_of_date(self.rustc_stamp_path, self.key))

    def test_dates_are_different(self):
        """Return True when the dates are different"""
        with open(self.rustc_stamp_path, "w") as rustc_stamp:
            rustc_stamp.write("2017-06-14None")
        self.assertTrue(self.build.program_out_of_date(self.rustc_stamp_path, self.key))

    def test_same_dates(self):
        """Return False both dates match"""
        with open(self.rustc_stamp_path, "w") as rustc_stamp:
            rustc_stamp.write("2017-06-15None")
        self.assertFalse(self.build.program_out_of_date(self.rustc_stamp_path, self.key))


class GenerateAndParseConfig(unittest.TestCase):
    """Test that we can serialize and deserialize a config.toml file"""
    def serialize_and_parse(self, args):
        from io import StringIO

        section_order, sections, targets = configure.parse_args(args)
        buffer = StringIO()
        configure.write_config_toml(buffer, section_order, targets, sections)
        build = bootstrap.RustBuild()
        build.config_toml = buffer.getvalue()

        try:
            import tomllib
            # Verify this is actually valid TOML.
            tomllib.loads(build.config_toml)
        except ImportError:
            print("warning: skipping TOML validation, need at least python 3.11", file=sys.stderr)
        return build

    def test_no_args(self):
        build = self.serialize_and_parse([])
        self.assertEqual(build.get_toml("changelog-seen"), '2')
        self.assertEqual(build.get_toml("profile"), 'user')
        self.assertIsNone(build.get_toml("llvm.download-ci-llvm"))

    def test_set_section(self):
        build = self.serialize_and_parse(["--set", "llvm.download-ci-llvm"])
        self.assertEqual(build.get_toml("download-ci-llvm", section="llvm"), 'true')

    def test_set_target(self):
        build = self.serialize_and_parse(["--set", "target.x86_64-unknown-linux-gnu.cc=gcc"])
        self.assertEqual(build.get_toml("cc", section="target.x86_64-unknown-linux-gnu"), 'gcc')

    def test_set_top_level(self):
        build = self.serialize_and_parse(["--set", "profile=compiler"])
        self.assertEqual(build.get_toml("profile"), 'compiler')

    def test_set_codegen_backends(self):
        build = self.serialize_and_parse(["--set", "rust.codegen-backends=cranelift"])
        self.assertNotEqual(build.config_toml.find("codegen-backends = ['cranelift']"), -1)
        build = self.serialize_and_parse(["--set", "rust.codegen-backends=cranelift,llvm"])
        self.assertNotEqual(build.config_toml.find("codegen-backends = ['cranelift', 'llvm']"), -1)
        build = self.serialize_and_parse(["--enable-full-tools"])
        self.assertNotEqual(build.config_toml.find("codegen-backends = ['llvm']"), -1)

