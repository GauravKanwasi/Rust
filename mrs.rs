class Solution:
    def lexGreaterPermutation(self, s: str, target: str) -> str:
        n = len(s)
        cnt = [0] * 26
        for ch in s:
            cnt[ord(ch) - 97] += 1

        t = [ord(ch) - 97 for ch in target]

        prefix = 0
        while prefix < n and cnt[t[prefix]]:
            cnt[t[prefix]] -= 1
            prefix += 1

        for i in range(prefix, -1, -1):
            if i < n:
                for c in range(t[i] + 1, 26):
                    if cnt[c]:
                        cnt[c] -= 1
                        rest = ''.join(chr(97 + d) * cnt[d] for d in range(26))
                        return target[:i] + chr(97 + c) + rest
            if i:
                cnt[t[i - 1]] += 1

        return ""
