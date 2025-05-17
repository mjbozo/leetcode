package main

import "fmt"

func main() {
	nums := []int{2, 0, 2, 1, 1, 0}
	sortColors(nums)
	fmt.Printf("Result = %v\n", nums)
}

func sortColors(nums []int) {
	red, white, blue := 0, 0, 0
	for _, num := range nums {
		switch num {
		case 0:
			red++
		case 1:
			white++
		case 2:
			blue++
		}
	}

	for i := range len(nums) {
		if i < red {
			nums[i] = 0
		} else if i < red+white {
			nums[i] = 1
		} else {
			nums[i] = 2
		}
	}
}
