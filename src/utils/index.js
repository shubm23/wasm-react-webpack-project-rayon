function isPrime(n) {
    if (n < 2) return false;
    for (let i = 2; i <= Math.sqrt(n); i++) {
        if (n % i === 0) return false;
    }
    return true;
}

function average(arr) {
    return arr.reduce((a, b) => a + b, 0) / arr.length;
}

function isPartOfSequence(index, arr) {
    return (arr[index] === arr[index - 1] + 1) || (arr[index] === arr[index + 1] - 1);
}

export function complexFilter(arr) {
    const avg = average(arr);
    return arr.filter((num, i) => {
        return isPrime(num) && num > avg && !isPartOfSequence(i, arr);
    });
}

export function parallelJs(n) {
    const start = performance.now();
    const numbers = Array.from({ length: n  }, (_, i) => i + 1);
    console.log(numbers.map(n => n * n));
    console.log("Processed", numbers.length, "items in Js", performance.now() - start, "ms");
}

export async function fetchParallel(n) {
    const start = performance.now();
    
    // Generate API URLs based on input `n`
    const urls = Array.from({ length: n }, (_, i) =>
        `https://jsonplaceholder.typicode.com/todos/${i + 1}`
    );
    
    // Fire off all requests in parallel
    const fetchPromises = urls.map(url =>
        fetch(url)
            .then(response => response.text())
            .catch(() => "Error")
    );
    
    const results = await Promise.all(fetchPromises);
    
    const end = performance.now();
    console.log(`Fetched ${n} APIs in JS in ${(end - start).toFixed(2)} ms`);
    
    return results;
};
