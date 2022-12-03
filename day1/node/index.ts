const input = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`

console.log(
  "first part solution:",
  input
    .split("\n\n")
    .map(e => e.split("\n")
      .map(e => Number(e))
      .reduce((a, b) => a + b)
    )
    .sort((a, b) => - a + b)[0]
);

console.log(
  "second part solution:",
  input
    .split("\n\n")
    .map(e => e.split("\n")
      .map(e => Number(e))
      .reduce((a, b) => a + b)
    )
    .sort((a, b) => - a + b)
    .slice(0, 3)
    .reduce((a, b) => a + b)
);
