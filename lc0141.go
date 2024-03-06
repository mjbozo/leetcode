// LeetCode problem 0141: Linked List Cycle
// https://leetcode.com/problems/linked-list-cycle/description/

package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func main() {
	head := ListNode{Val: 3}
	node1 := ListNode{Val: 2}
	rest := ListNode{Val: 0, Next: &ListNode{Val: -4, Next: &node1}}
	head.Next = &node1
	node1.Next = &rest

	result := hasCycle(&head)
	fmt.Printf("Result = %v\n", result)
}

func hasCycle(head *ListNode) bool {
	slow := head
	fast := head

	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next

		if slow == fast {
			return true
		}
	}

	return false
}
