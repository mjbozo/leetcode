// LeetCode problem 3043: Find the Length of the Longest Common Prefix
// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/description/

package main

import (
	"fmt"
	"strconv"
)

func main() {
	result := longestCommonPrefix([]int{1, 2, 3}, []int{4, 4, 4})
	fmt.Printf("Result = %d\n", result)
}

type TrieNode struct {
	val      byte
	children []*TrieNode
}

func (t *TrieNode) push(num string) {
	if len(num) == 0 {
		return
	}

	for _, child := range t.children {
		if child.val == num[0] {
			child.push(num[1:])
			return
		}
	}

	t.branch(num)
}

func (t *TrieNode) branch(num string) {
	if len(num) == 0 {
		return
	}

	newNode := &TrieNode{val: num[0]}
	t.children = append(t.children, newNode)
	newNode.branch(num[1:])
}

func (t *TrieNode) find(num string) int {
	if len(num) == 0 {
		return 0
	}

	depth := 0
	for _, child := range t.children {
		if child.val == num[0] {
			depth = 1
			depth += child.find(num[1:])
		}
	}

	return depth
}

func longestCommonPrefix(arr1 []int, arr2 []int) int {
	trie := &TrieNode{}
	for _, num := range arr1 {
		trie.push(strconv.Itoa(num))
	}

	fmt.Printf("%q\n", trie)

	maxDepth := 0
	for _, num := range arr2 {
		depth := trie.find(strconv.Itoa(num))
		if depth > maxDepth {
			maxDepth = depth
		}
	}

	return maxDepth
}
