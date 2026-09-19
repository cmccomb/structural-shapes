"""Regression checks for missing data and designation interpretation."""

import csv
from decimal import InvalidOperation
import io
import unittest

import generate_aisc as catalog


class CatalogValidationTests(unittest.TestCase):
    def test_double_angle_fraction_and_orientation(self):
        row = {"AISC_Manual_Label": "2L2-1/2X1-1/2X3/16X3/4SLBB", "d": "1.5", "b": "2.5", "t": "0.188"}
        self.assertEqual(catalog.double_angle_gap(row), "0.75")
        row.update(AISC_Manual_Label="2L12X12X1-3/8X1-1/2", d="12", b="12", t="1.38")
        self.assertEqual(catalog.double_angle_gap(row), "1.5")
        row["AISC_Manual_Label"] = "2L12X12X1-3/8"
        self.assertEqual(catalog.double_angle_gap(row), "0.0")

    def test_double_angle_rejects_missing_or_inconsistent_orientation(self):
        row = {"AISC_Manual_Label": "2L8X4X1/2", "d": "8", "b": "4", "t": "0.5"}
        with self.assertRaisesRegex(ValueError, "orientation"):
            catalog.double_angle_gap(row)
        row["AISC_Manual_Label"] += "SLBB"
        with self.assertRaisesRegex(ValueError, "orientation"):
            catalog.double_angle_gap(row)

    def test_missing_or_nonphysical_properties_are_rejected(self):
        for value in ("", "NaN", "Infinity", "0", "-1"):
            with self.subTest(value=value), self.assertRaises((ValueError, InvalidOperation)):
                catalog.number({"J": value, "AISC_Manual_Label": "W12X26"}, "J")

    def test_case_insensitive_alias_collisions_are_rejected(self):
        rows = list(csv.DictReader(io.StringIO(catalog.CSV_PATH.read_text())))
        rows[1]["EDI_Std_Nomenclature"] = rows[0]["AISC_Manual_Label"].lower()
        output = io.StringIO()
        writer = csv.DictWriter(output, fieldnames=catalog.FIELDS)
        writer.writeheader()
        writer.writerows(rows)
        with self.assertRaisesRegex(ValueError, "Ambiguous designation"):
            catalog.generate(output.getvalue())


if __name__ == "__main__":
    unittest.main()
