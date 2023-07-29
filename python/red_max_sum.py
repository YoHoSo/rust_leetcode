'''
小红拿到了一个数组，她希望进行最多一次操作：将一个元素修改为x。小红想知道，最终的连续子数组最大和最大是多少？



输入描述
第一行输入一个正整数t，代表询问次数。

对于每次询问，输入两行：

第一行输入两个正整数n和x。代表数组的大小，以及小红可以修改成的元素。

第二行输入n个正整数a_i，代表小红拿到的数组。

1 ≤ t ≤ 100000

1 ≤ n ≤ 200000

-10^9 ≤ x ,a_i ≤ 10^9

每组所有询问的n的和不超过200000。

输出描述
输出t行，每行输出一个整数，代表连续子数组的最大和。

样例输入
3
5 10
5 -1 -5 -3 2
2 -3
-5 -2
6 10
4 -2 -11 -1 4 -1
样例输出
15
-2
15

提示
第一组询问，修改第二个数。

第二组询问，不进行任何修改。

第三组询问，修改第三个数。
'''

def get_sum(arr:list, x:int) -> int:
    n = len(arr)
    max_ending_here = max_so_far = arr[0]
    max_modified_sum = max(arr[0] + x, x)

    for i in range(1, n):
        max_ending_here = max(arr[i], max_ending_here + arr[i])
        max_so_far = max(max_so_far, max_ending_here)

        # Calculate the maximum subarray sum after one modification
        max_modified_sum = max(max_modified_sum, max(arr[i] + x, max_so_far))

    return max_modified_sum

if __name__ == '__main__':
    # t = int(input())
    # for _ in range(t):
    #     n, x = map(int, input().split())
    #     arr = list(map(int, input().split()))
    #     print(get_sum(arr, x))
    
    print(get_sum([5, -1, -5, -3, 2], 10))
    print(get_sum([-5, -2], -3))
    print(get_sum([4, -2, -11, -1, 4, -1], 10))