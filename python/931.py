from typing import List
class Solution:
    def minFallingPathSum(self, matrix: List[List[int]]) -> int:
        cost_matrix = [[0 for _ in range(len(matrix[0]))] for _ in range(len(matrix))]
        # init the first row
        for i in range(len(matrix[0])):
            cost_matrix[0][i] = matrix[0][i]
            
        # calculate the cost matrix from the second row
        for i in range(1, len(matrix)):
            for j in range(len(matrix[0])):
                cost_matrix[i][j] = matrix[i][j] + min(
                    cost_matrix[i-1][j-1] if j > 0 else float('inf'),  
                    cost_matrix[i-1][j],
                    cost_matrix[i-1][j+1] if j < len(matrix[0])-1 else float('inf')
                )
        return min(cost_matrix[-1])
    
if __name__ == '__main__':
    print(Solution().minFallingPathSum([[2,1,3],[6,5,4],[7,8,9]]))
    print(Solution().minFallingPathSum([[-19,57],[-40,-5]]))
    