class TrieNode:
    def __init__(self):
        self.children = {}
        self.isEndOfWord = False

class AutocompleteService:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, username):
        node = self.root
        for char in username:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
        node.isEndOfWord = True

    def contains(self, username):
        node = self.root
        for char in username:
            if char not in node.children:
                return False
            node = node.children[char]
        return node.isEndOfWord

    def startsWith(self, prefix):
        def dfs(node, pre):
            if node.isEndOfWord:
                result.append(pre)
            for char in node.children:
                dfs(node.children[char], pre + char)

        result = []
        node = self.root
        for char in prefix:
            if char not in node.children:
                return []
            node = node.children[char]

        dfs(node, prefix)
        return result
