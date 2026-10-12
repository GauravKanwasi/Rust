class Solution:
    def lexGreaterPermutation(self, s: str, target: str) -> str:
        n = len(s)
        cnt = [0] * 26
        for ch in s:
            cnt[ord(ch) - 97] += 1

        t = [ord(ch) - 97 for ch in target]

        i = 0
        while i < n and cnt[t[i]] > 0:
            cnt[t[i]] -= 1
            i += 1

        while True:
            if i < n:
                for c in range(t[i] + 1, 26):
                    if cnt[c]:
                        cnt[c] -= 1
                        suffix = ''.join(chr(97 + d) * cnt[d] for d in range(26) if cnt[d])
                        return target[:i] + chr(97 + c) + suffix
            if i == 0:
                return ""
            i -= 1
            cnt[t[i]] += 1
