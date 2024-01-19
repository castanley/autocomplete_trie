import unittest
from autocomplete_service import AutocompleteService

class TestAutocompleteService(unittest.TestCase):
    def setUp(self):
        self.service = AutocompleteService()
        self.usernames = ["alice", "albert", "bob", "carol", "caroline"]

        for username in self.usernames:
            self.service.insert(username)

    def test_contains(self):
        self.assertTrue(self.service.contains("alice"))
        self.assertFalse(self.service.contains("alex"))

    def test_startsWith(self):
        self.assertListEqual(sorted(self.service.startsWith("a")), sorted(["alice", "albert"]))
        self.assertListEqual(sorted(self.service.startsWith("car")), sorted(["carol", "caroline"]))
        self.assertListEqual(self.service.startsWith("d"), [])

    def test_edgeCases(self):
        self.assertListEqual(self.service.startsWith(""), [])
        self.assertFalse(self.service.contains(""))

if __name__ == '__main__':
    unittest.main()
