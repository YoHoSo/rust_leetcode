# 给你一个链表的头节点 head ，判断链表中是否有环。

# 如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。 为了表示给定链表中的环，评测系统内部使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。注意：pos 不作为参数进行传递 。仅仅是为了标识链表的实际情况。

# 如果链表中存在环 ，则返回 true 。 否则，返回 false 。

# Definition for singly-linked list.

from typing import *

class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        if head == None or head.next == None:
            return False
        
        id_set = set()
        while head.next != None:
            if id(head.next) in id_set:
                return True
            else:
                id_set.add(id(head))
                
            head = head.next
        return False
    
    def hasCycleOfficial(self, head: Optional[ListNode]) -> bool:
        seen = set()
        while head:
            if head in seen:
                return True
            seen.add(head)
            head = head.next
        return False
    
    def fastSlowPtr(self, head: Optional[ListNode]) -> bool:
        if head == None or head.next == None:
            return False
        
        fast = head.next
        slow = head
        
        while slow != fast:
            if fast == None or fast.next == None:
                return False
            slow = slow.next
            fast = fast.next.next
        return True
        

    
if __name__ == "__main__":
    node4 = ListNode(-4)
    node3 = ListNode(0)
    node2 = ListNode(2)
    node1 = ListNode(3)
    
    node1.next = node2
    node2.next = node3
    node3.next = node4
    node4.next = node2
    
    print(Solution().fastSlowPtr(node1))
    
    node1.next = node2
    node2.next = node1
    print(Solution().fastSlowPtr(node1))
    
    node1.next = None
    print(Solution().fastSlowPtr(node1))