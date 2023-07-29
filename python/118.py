# 118. 杨辉三角
# 给定一个非负整数 numRows，生成「杨辉三角」的前 numRows 行。

# 在「杨辉三角」中，每个数是它左上方和右上方的数的和。

from typing import *

class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        ans = [[1] * i for i in range(1, numRows+1)]
        if numRows < 3:
            return ans
        
        for i in range(2, numRows):
            for j in range(1, len(ans[i])-1):
                ans[i][j] = ans[i-1][j-1] + ans[i-1][j]
                
        return ans
    
    
if __name__ == "__main__":
    print(Solution().generate(1))
    print(Solution().generate(2))
    print(Solution().generate(3))
    print(Solution().generate(4))
    print(Solution().generate(5))

