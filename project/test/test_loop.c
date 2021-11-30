#include <stdio.h>

int check(int i) {
	return i == 747;
}

int main() {
	int n;

	while (1) {
		scanf("%d", &n);

		if (check(n)) {
			printf("Correct value!\n");
			break;
		} else {
			printf("Try again");
		}
	}
	return 0;
}
