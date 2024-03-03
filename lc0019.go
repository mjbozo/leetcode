// LeetCode problem 0023: Merge K Sorted Lists
// https://leetcode.com/problems/merge-k-sorted-lists/description/

package lc0019

import (
	"fmt"
)

type ListNode struct {
	Val  int32
	Next *ListNode
}

func (l *ListNode) ToString() string {
	var buf string
	buf += fmt.Sprintf("ListNode { Val = %v, Next = ", l.Val)
	if l.Next != nil {
		buf += l.Next.ToString()
	} else {
		buf += "nil }\n"
	}

	return buf
}

func main() {
	list := ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4, Next: &ListNode{Val: 5}}}}}
	result := removeNthFromEnd(&list, 5)
	fmt.Printf("%v\n", result.ToString())
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	length := 0
	current := head
	for current != nil {
		current = current.Next
		length++
	}

	if length == n {
		return head.Next
	}

	h := head
	current = h
	for i := 0; i < length; i++ {
		if i == length-n-1 {
			current.Next = current.Next.Next
			break
		} else {
			current = current.Next
		}
	}

	return h
}
