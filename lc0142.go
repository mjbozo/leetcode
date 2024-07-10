// LeetCode problem 0142: Linked List Cycle II
// https://leetcode.com/problems/linked-list-cycle-ii/description/

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
	fmt.Printf("Result = %v\n", result.Val)
}

func hasCycle(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return nil
	}

	slow := head
	fast := head.Next.Next

	if slow == fast {
		return head
	}

	index := 0
	for fast != nil && fast.Next != nil {
		index++
		slow = slow.Next
		fast = fast.Next.Next

		if slow == fast {
			return slow
		}
	}

	return nil
}
