# 740. 删除并获得点数
# 给你一个整数数组 nums ，你可以对它进行一些操作。

# 每次操作中，选择任意一个 nums[i] ，删除它并获得 nums[i] 的点数。之后，你必须删除 所有 等于 nums[i] - 1 和 nums[i] + 1 的元素。

# 开始你拥有 0 个点数。返回你能通过这些操作获得的最大点数。
from typing import *


class Solution:
    def deleteAndEarn(self, nums: List[int]) -> int:
        maxVal = max(nums)
        total = [0] * (maxVal + 1)
        for val in nums:
            total[val] += val

        print(total)

    
    
if __name__ == "__main__":
    print(Solution().deleteAndEarn([3,4,2]))
    print(Solution().deleteAndEarn([2,2,3,3,3,4]))