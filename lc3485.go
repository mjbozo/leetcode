// LeetCode problem 3485: Longest Common Prefix of K Strings After Removal
// https://leetcode.com/problems/longest-common-prefix-of-k-strings-after-removal/description/

package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val   int
	count int
	next  map[int]*Node
}

func (n *Node) AddWord(word string) {
	if len(word) == 0 {
		return
	}

	val := int(word[0])
	if next, ok := n.next[val]; ok {
		next.count++
		next.AddWord(word[1:])
	} else {
		next := &Node{val: val, count: 1, next: make(map[int]*Node)}
		n.next[val] = next
		next.AddWord(word[1:])
	}
}

func (n *Node) DecrementWord(word string) {
	n.count--

	if len(word) == 0 {
		return
	}

	c := word[0]
	n.next[int(c)].DecrementWord(word[1:])
}

func (n Node) Count(k int) (string, int) {
	depth := 0
	chars := ""

	for _, next := range n.next {
		if next.count >= k {
			nextC, nextD := next.Count(k)
			nextDepth := nextD + 1
			if nextDepth > depth {
				chars = nextC
				depth = nextDepth
			}
		}
	}

	return string(n.val) + chars, depth
}

func main() {
	result := longestCommonPrefix([]string{"jump", "run", "run", "jump", "run"}, 2)
	fmt.Printf("Result = %d\n", result)
}

func longestCommonPrefix(words []string, k int) []int {
	graph := Node{val: 0, count: 0, next: make(map[int]*Node)}

	for _, word := range words {
		graph.AddWord(word)
	}

	longest, longestDepth := graph.Count(k)
	longest = longest[1:]
	graph.DecrementWord(longest)
	_, secondDepth := graph.Count(k)

	prefixes := make([]int, 0)
	for _, word := range words {
		if strings.HasPrefix(word, longest) {
			prefixes = append(prefixes, secondDepth)
		} else {
			prefixes = append(prefixes, longestDepth)
		}
	}

	return prefixes
}
