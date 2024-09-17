// LeetCode problem 3217: Delete Nodes From Linked List Present in Array
// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/description/

package main

import "fmt"

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
	result := modifiedList([]int{1, 7, 6, 2, 4}, build([]int{3, 7, 1, 8, 1}))
	fmt.Printf("Result = %s\n", result.toString())
}

func modifiedList(nums []int, head *ListNode) *ListNode {
	numsMap := make(map[int]bool)
	for _, num := range nums {
		numsMap[num] = true
	}

	for numsMap[head.Val] {
		head = head.Next
	}

	current := head
	for current != nil && current.Next != nil {
		for numsMap[current.Next.Val] {
			current.Next = current.Next.Next
			if current.Next == nil {
				break
			}
		}
		current = current.Next
	}

	return head
}
