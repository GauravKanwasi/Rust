class Solution:
    def lexGreaterPermutation(self, s: str, target: str) -> str:
        n = len(s)
        cnt = [0] * 26
        for ch in s:
            cnt[ord(ch) - 97] += 1

        i = 0
        while i < n:
            idx = ord(target[i]) - 97
            if cnt[idx] > 0:
                cnt[idx] -= 1
                i += 1
            else:
                break

        while i >= 0:
            if i < n:
                t_idx = ord(target[i]) - 97
                chosen = -1
                for c in range(t_idx + 1, 26):
                    if cnt[c] > 0:
                        chosen = c
                        break
                if chosen != -1:
                    cnt[chosen] -= 1
                    prefix = target[:i] + chr(97 + chosen)
                    suffix = []
                    for c in range(26):
                        if cnt[c]:
                            suffix.append(chr(97 + c) * cnt[c])
                    return prefix + ''.join(suffix)
            if i == 0:
                break
            prev_idx = ord(target[i - 1]) - 97
            cnt[prev_idx] += 1
            i -= 1

        return ""
