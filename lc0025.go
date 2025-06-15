// LeetCode problem 0025: Reverse Nodes in k-Group
// https://leetcode.com/problems/reverse-nodes-in-k-group/description/

package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func main() {
	result := reverseKGroup()
	fmt.Printf("Hello\n")
}

func reverseKGroup(head *ListNode, k int) *ListNode {
	if k == 1 {
		return head
	}

	stack := make([]*ListNode, 0, k)

	current := head
	for i := 0; i < k; i++ {
		if current != nil {
			stack = append(stack, current)
			current = current.Next
		} else {
			return head
		}
	}

	new_head := stack[len(stack)-1]
	stack = stack[:len(stack)-1]
	new_current := new_head

	for new_current != nil {
		next := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		new_current.Next = next
		new_current = next

		if len(stack) == 0 {
			break
		}
	}

	if new_current != nil {
		new_current.Next = reverseKGroup(current, k)
	}

	return new_head
}
