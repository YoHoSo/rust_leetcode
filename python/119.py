# 119. 杨辉三角 II
# 给定一个非负索引 rowIndex，返回「杨辉三角」的第 rowIndex 行。

# 在「杨辉三角」中，每个数是它左上方和右上方的数的和。

# 示例 1:

# 输入: rowIndex = 3
# 输出: [1,3,3,1]
# 示例 2:

# 输入: rowIndex = 0
# 输出: [1]
# 示例 3:

# 输入: rowIndex = 1
# 输出: [1,1]
from typing import *


class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        if rowIndex < 2:
            return [1] * (rowIndex+1)
        
        prev = [1, 1]
        currentIndex = 2
        while currentIndex <= rowIndex:
            current = [1] * (currentIndex+1)
            for i in range(1, len(current)-1):
                current[i] = prev[i-1] + prev[i]
            prev = current
            currentIndex += 1
            
        return prev


# class Solution {
# public:
#     vector<int> getRow(int rowIndex) {
#         vector<int> row(rowIndex + 1);
#         row[0] = 1;
#         for (int i = 1; i <= rowIndex; ++i) {
#             row[i] = 1LL * row[i - 1] * (rowIndex - i + 1) / i;
#!            线性递推, next = prev * (n-m+1)/m
#         }
#         return row;
#     }
# };

# 作者：力扣官方题解
# 链接：https://leetcode.cn/problems/pascals-triangle-ii/solutions/601082/yang-hui-san-jiao-ii-by-leetcode-solutio-shuk/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

if __name__ == "__main__":
    print(Solution().getRow(0))
    print(Solution().getRow(1))
    print(Solution().getRow(2))
    print(Solution().getRow(3))
    print(Solution().getRow(4))