# 小红书的推荐帖子列表为[0,n]，其中所有的帖子初始状态为”普通“，现在运营同学把其中的一些帖子区间标记为了”精华“。

# 第一行输入两个正整数n,m,k，代表初始帖子列表长度，精华区间的数量，以及运营同学准备截取的长度。

# 接下来的m行，每行输入两个正整数li,ri，代表第i个区间。

# 1 ≤ k ≤ n ≤ 1000000000

# 1 ≤ m ≤ 100000

# 0 ≤ li < ri ≤ n

# 保证任意两个区间是不重叠的。

# 一个正整数，代表最多的精华帖子数量。

# input
# 5 2 3
# 1 2
# 3 5

# output
# 2
def max_good_post(n:int, m:int, k:int, intervals:list) -> int:
    # init the post list

    # sort the intervals by the left index, from small to large
    # intervals.sort(key=lambda x:x[0])
    # print(intervals)
    
    # # calculate adjacent intervals distance
    # dist = [0] * (m-1)
    # for i in range(m-1):
    #     dist[i] = intervals[i+1][0] - intervals[i][1]
    # print(dist)
    
    # interval_sizes = [0] * m
    # for i in range(m):
    #     interval_sizes[i] = intervals[i][1] - intervals[i][0]
    # print(interval_sizes)
    
    posts = [0] * n
    for i in range(m):
        li, ri = intervals[i]
        for j in range(li, ri):
            posts[j] = 1
    print(posts) 
    # get the max good post
    max_good_post = 0
    for i in range(n-k+1):
        # max_good_post = max(max_good_post, sum(posts[i:i+k]))
        
        running_sum = 0
        running_max = 0
        for j in range(i, i+k):
            if posts[j] == 1:
                running_sum += 1
            if posts[j] == 0:
                running_max = max(running_max, running_sum)
                running_sum = 0
        running_max = max(running_max, running_sum)
    
        max_good_post = max(max_good_post, running_max)
        print("intercal: ", posts[i:i+k], "running_max: ", running_max)
                
    return max_good_post
    


if __name__ == "__main__":
    n,m,k = 5,2,3
    intervals = [[1,2],[3,5]]
    print(max_good_post(n,m,k,intervals))
    # n,m,k = map(int, input().split())
    # intervals = []
    # for _ in range(m):
    #     intervals.append(list(map(int, input().split())))
    # print(max_good_post(n,m,k,intervals))