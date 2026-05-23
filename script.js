function doStuff() {
    console.log("Doing stuff")
    // getElementById

    // setInnerHTML



}

function toDecimal(str, base) {
    const digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    str = str.toUpperCase();

    let value = 0;
    let power = 0;

    for (let i = str.length - 1; i >= 0; i--) {
        const char = str[i];
        const digit = digits.indexOf(char);

        if (digit === -1 || digit >= Math.abs(base)) {
            throw new Error(`Invalid digit '${char}' for base ${base}`);
        }

        value += digit * Math.pow(base, power);
        power++;
    }

    return value;
}

function fromDecimal(num, base) {
    if (base === 0 || base === 1 || base === -1) {
        throw new Error("Unsupported base");
    }

    const digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if (num === 0) return "0";

    let result = "";
    let n = num;

    if (base > 0) {
        while (n > 0) {
            const remainder = n % base;
            result = digits[remainder] + result;
            n = Math.floor(n / base);
        }
    } else {
        while (n !== 0) {
            let remainder = n % base;
            n = Math.trunc(n / base);

            if (remainder < 0) {
                remainder += Math.abs(base);
                n += 1;
            }

            result = digits[remainder] + result;
        }
    }

    return result;
}