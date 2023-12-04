package day01

import (
	"testing"
)

func TestPartOne(t *testing.T) {
	tests := []struct {
		name string
		want int
	}{
		{"Example", 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got, err := PartOne(); got != tt.want {
				t.Errorf("PartOne() = %v, want %v", got, tt.want)
			} else if err != nil {
				t.Errorf("PartOne() error = %v", err)
			}
		})
	}
}

func TestPartTwo(t *testing.T) {
	tests := []struct {
		name string
		want int
	}{
		{"Example", 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got, err := PartTwo(); got != tt.want {
				t.Errorf("PartTwo() = %v, want %v", got, tt.want)
			} else if err != nil {
				t.Errorf("PartTwo() error = %v", err)
			}
		})
	}
}
