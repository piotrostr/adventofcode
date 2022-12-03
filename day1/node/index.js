var input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
console.log("first part solution", input
    .split("\n\n")
    .map(function (e) { return e.split("\n")
    .map(function (e) { return Number(e); })
    .reduce(function (a, b) { return a + b; }); })
    .sort(function (a, b) { return -a + b; })[0]);
console.log("second part solution", input
    .split("\n\n")
    .map(function (e) { return e.split("\n")
    .map(function (e) { return Number(e); })
    .reduce(function (a, b) { return a + b; }); })
    .sort(function (a, b) { return -a + b; })
    .slice(0, 3)
    .reduce(function (a, b) { return a + b; }));
