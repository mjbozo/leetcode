// LeetCode problem 0206: Reverse Linked List
// https://leetcode.com/problems/reverse-linked-list/description/

package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func (l *ListNode) toString() string {
	if l == nil {
		return "nil"
	}

	return fmt.Sprintf("%d, %v", l.Val, l.Next.toString())
}

func build(vals []int) *ListNode {
	if len(vals) == 0 {
		return nil
	}

	node := ListNode{Val: vals[0]}
	node.Next = build(vals[1:])
	return &node
}

func main() {
	list := build([]int{1, 2, 3, 4, 5})
	result := reverseList(list)
	fmt.Printf("%v\n", result.toString())
}

func reverseList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	var prev *ListNode
	current := head
	next := current.Next

	for current != nil {
		next = current.Next
		current.Next = prev
		prev = current
		current = next
	}

	return prev
}
