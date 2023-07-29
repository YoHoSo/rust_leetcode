# 你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，
# 影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，
# 如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。

# 给定一个代表每个房屋存放金额的非负整数数组，计算你不触动警报装置的情况下 
# ，一夜之内能够偷窃到的最高金额。

# 示例 1：

# 输入：[1,2,3,1]
# 输出：4
# 解释：偷窃 1 号房屋 (金额 = 1) ，然后偷窃 3 号房屋 (金额 = 3)。
#      偷窃到的最高金额 = 1 + 3 = 4 。
# 示例 2：

# 输入：[2,7,9,3,1]
# 输出：12
# 解释：偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
#      偷窃到的最高金额 = 2 + 9 + 1 = 12 。

from typing import *

class Solution:
    def rob_not_pass(self, nums: List[int]) -> int:
        if len(nums) < 3:
            return max(nums)
        
        max_val = max(nums[:2])
        prev_max_val = nums[0]
        print(prev_max_val, max_val)
        for i in range(2, len(nums)):
            if nums[i]+prev_max_val > max_val:
                old_max_val = max_val
                max_val = nums[i]+prev_max_val
                prev_max_val = old_max_val
                
            print(prev_max_val, max_val)
        
        return max_val
    
    def rob(self, nums: List[int]) -> int:
        if len(nums) < 3:
            return max(nums)
        
        dp = [0] * len(nums)
        dp[0] = nums[0]
        dp[1] = max(nums[:2])
        
        for i in range(2, len(nums)):
            dp[i] = max(nums[i]+dp[i-2], dp[i-1])

        # print(dp)
        return dp[-1]
    


if __name__ == "__main__":
    # print(Solution().rob([1,2,3,1]))
    print(Solution().rob([2,7,9,3,1])) 