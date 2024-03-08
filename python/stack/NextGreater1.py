def nextGreaterElement(nums1, nums2):
        d = {v: i for i, v in enumerate(nums1)}
        res = [-1] * len(nums1)
        stack = []

        for i in range(len(nums2)):
            while stack and stack[-1] < nums2[i]:
                res[d[stack.pop()]] = nums2[i]
            if nums2[i] in d:
                stack.append(nums2[i])
        return res
