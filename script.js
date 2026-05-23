function doStuff() {
    console.log("Doing stuff");

    try {
        const number = document.getElementById("numberInput").value.trim();
        const originbase = parseInt(document.getElementById("numberBase").value);
        const convbase = parseInt(document.getElementById("convBase").value);

        const decimal = toDecimal(number, originbase);
        const result = fromDecimal(decimal, convbase);

        document.getElementById("result").innerHTML = `Result: ${result}`;
    }
    catch (err) {
        document.getElementById("result").innerHTML = `Error: ${err.message}`;
        console.error(err);
    }
}

// ---------- conversion functions ----------

function toDecimal(str, base) {
    const digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    str = str.toUpperCase();

    let value = 0;
    let power = 0;

    for (let i = str.length - 1; i >= 0; i--) {
        const digit = digits.indexOf(str[i]);

        if (digit === -1 || digit >= Math.abs(base)) {
            throw new Error(`Invalid digit '${str[i]}' for base ${base}`);
        }

        value += digit * Math.pow(base, power);
        power++;
    }

    return value;
}

function fromDecimal(num, base) {
    const digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if (base === 0 || base === 1 || base === -1) {
        throw new Error("Unsupported base");
    }

    if (num === 0) return "0";

    let result = "";
    let n = num;

    // handle negative bases AND negative numbers correctly
    while (n !== 0) {
        let remainder = n % base;
        n = Math.trunc(n / base);

        // fix negative remainder case
        if (remainder < 0) {
            remainder += Math.abs(base);
            n += 1;
        }

        result = digits[remainder] + result;
    }

    return result;
}