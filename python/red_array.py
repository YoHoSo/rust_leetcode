# 小红希望你构造一个数组满足以下条件：

# 1. 数组共有n个元素，且所有元素两两不相等。

# 2. 所有元素的最大公约数等于k。

# 3. 所有元素之和尽可能小。

# 请你输出数组元素之和的最小值。

def get_min_red_array_sum(n:int, k:int) -> int:
    arr = [0] * n
    for i in range(n):
        arr[i] = k * (i+1)
    print(arr)
    
    return sum(arr)

if __name__ == '__main__':
    # n, k = map(int, input().split())
    # print(get_min_red_array_sum(n, k))
    n, k = 3, 1
    print(get_min_red_array_sum(n, k))
    n, k = 2, 2
    print(get_min_red_array_sum(n, k))