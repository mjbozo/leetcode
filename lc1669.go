// LeetCode problem 1669: Merge In Between Linked Lists
// https://leetcode.com/problems/merge-in-between-linked-lists/description/

package lc1669

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
	// list1 := ListNode{Val: 10, Next: ListNode{Val: 1, Next: ListNode{Val: 13, Next: ListNode{Val: 6, Next: ListNode{Val: 9, Next: ListNode{Val: 5, Next: nil}}}}}}
	list1 := build([]int{10, 1, 13, 6, 9, 5})
	list2 := build([]int{1000000, 1000001, 1000002})
	result := mergeInBetween(list1, 3, 4, list2)
	fmt.Printf("%v\n", result.toString())
}

func mergeInBetween(list1 *ListNode, a int, b int, list2 *ListNode) *ListNode {
	nodeA := list1
	for i := 0; i < a-1; i++ {
		nodeA = nodeA.Next
	}

	nodeB := nodeA.Next
	for i := a; i < b+1; i++ {
		nodeB = nodeB.Next
	}

	nodeC := list2
	for nodeC.Next != nil {
		nodeC = nodeC.Next
	}

	nodeA.Next = list2
	nodeC.Next = nodeB
	return list1
}
