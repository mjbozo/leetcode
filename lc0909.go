// LeetCode problem 0909: Snakes and Ladders
// https://leetcode.com/problems/snakes-and-ladders/description/

package main

import (
	"fmt"
	"slices"
)

func main() {
	board := [][]int{
		[]int{-1, -1, -1, -1, -1, -1},
		[]int{-1, -1, -1, -1, -1, -1},
		[]int{-1, -1, -1, -1, -1, -1},
		[]int{-1, 35, -1, -1, 13, -1},
		[]int{-1, -1, -1, -1, -1, -1},
		[]int{-1, 15, -1, -1, -1, -1},
	}

	board = [][]int{
		[]int{1, 1, -1},
		[]int{1, 1, 1},
		[]int{-1, 1, 1},
	}

	board = [][]int{
		[]int{-1, -1, 2, -1},
		[]int{14, 2, 12, 3},
		[]int{4, 9, 1, 11},
		[]int{-1, 2, 1, 16},
	}

	result := snakesAndLadders(board)
	fmt.Printf("Result = %d\n", result)
}

func snakesAndLadders(board [][]int) int {
	n := len(board)
	seen := make([]bool, n*n+1)
	queue := make(chan int, n*n)
	queue <- 1
	moves := 0

	for len(queue) > 0 {
		next := make([]int, 0)
		for len(queue) > 0 {
			s := <-queue
			fmt.Println(s)
			if s == n*n {
				return moves
			}

			for i := 1; i <= 6 && s+i <= n*n; i++ {
				k := s + i
				r, c := spaceToPos(board, k)
				if board[r][c] != -1 {
					k = board[r][c]
				}
				if !seen[k] {
					seen[k] = true
					next = append(next, k)
				}
			}
		}

		for _, nn := range next {
			queue <- nn
		}
		moves++
	}

	return -1
}

func spaceToPos(board [][]int, index int) (int, int) {
	n := len(board)
	r := n - ((index - 1) / n) - 1
	c := (index - 1) % n
	if n%2 == r%2 {
		c = n - 1 - c
	}
	return r, c
}
