package main

import (
	"fmt"
	"slices"
)

type Node struct {
	val      byte
	count    int
	children []*Node
}

func (n *Node) push(s string) {
	if len(s) == 0 {
		return
	}

	for _, child := range n.children {
		if child.val == s[0] {
			child.count += 1
			child.push(s[1:])
			return
		}
	}

	n.branch(s)
}

func (n *Node) branch(s string) {
	if len(s) == 0 {
		return
	}

	newNode := &Node{val: s[0], count: 1}
	n.children = append(n.children, newNode)
	newNode.branch(s[1:])
}

func (n *Node) find(word string) int {
	if len(word) == 0 {
		return 0
	}

	idx := slices.IndexFunc(n.children, func(o *Node) bool { return o.val == word[0] })
	if idx == -1 {
		panic("trie must be badly constructed")
	}

	child := n.children[idx]
	score := child.count

	score += child.find(word[1:])
	return score
}

func main() {
	scores := sumPrefixScores([]string{"abc", "ab", "bc", "b"})
	fmt.Println(scores)
}

func sumPrefixScores(words []string) []int {
	trie := &Node{}
	for _, word := range words {
		trie.push(word)
	}

	sumPrefixScores := make([]int, 0, len(words))
	for _, word := range words {
		score := trie.find(word)
		sumPrefixScores = append(sumPrefixScores, score)
	}

	return sumPrefixScores
}
