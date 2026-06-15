class Frac {
    constructor(n, d = 1) {
        if (d === 0) throw new Error("Denominator cannot be zero");
        // (-5/-7) -> (5/7),
        // (5/-7) -> (-5/7),
        // (-5/7 -> -5/7),
        // (5/7) -> (5/7)
        if (d < 0) {
            this.n = -n;
            this.d = -d;
        }
        else {
            this.n = n;
            this.d = d;
        }
        this.reduce();
    }

    reduce() {
        const gcd = (a, b) => (b === 0 ? Math.abs(a) : gcd(b, Math.abs(a) % Math.abs(b)));
        // absolute overload
        const d = gcd(this.n, this.d);
        this.n /= d;
        this.d /= d;
        return this;
    }

    value() {
        return this.n / this.d;
    }
}
const CHARS = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/';

function doStuff() {
    const input = document.getElementById('numberInput').value.trim();
    const base1 = parseInt(document.getElementById('numberBase').value);
    const base2 = parseInt(document.getElementById('convBase').value);

    // WAS: parseInt(c)
    const digits = input.split('').map(c => {
        if (c === '-' || c === '.') return c;
        return CHARS.indexOf(c);          // <-- changed
    });

    try {
        const result = base_to_base(digits, new Frac(base1), new Frac(base2));
        // WAS: result.join('')
        document.getElementById('result').textContent = result
            .map(x => typeof x === 'number' ? CHARS[x] : x)   // <-- changed
            .join('');
    } catch (e) {
        document.getElementById('result').textContent = 'Error: ' + e.message;
    }
}


function binary_to_fbase(number, fbase) {
    const baseValue = fbase.value();
    let n = fbase.n;
    let d = fbase.d;

    let integerDigits;
    let fractionalDigits = [];
    let isNegative = false;
    let integerPart, fractionalPart;

    if (baseValue > 0) {
        // strip sign, process, add minus sign at the end
        isNegative = number < 0;
        const positive = Math.abs(number);
        integerPart = Math.trunc(positive);
        fractionalPart = positive - integerPart;
    } else {
        // process directly
        integerPart = Math.trunc(number);
        fractionalPart = number - integerPart;
    }

    if (integerPart === 0) integerDigits = [0];
    else {
        integerDigits = [];
        while (integerPart !== 0) {
            let rem = integerPart % n;
            integerPart = Math.floor((integerPart - rem) / n) * d;
            integerDigits.push(rem);
        }
        integerDigits.reverse();
    }

    while (fractionalPart !== 0 && fractionalDigits.length < FLOAT_PRECISION) {
        let product = fractionalPart * baseValue;
        let digit = Math.floor(product);
        // just keep the fractional part
        fractionalPart = product - digit;
        // and add a digit
        fractionalDigits.push(digit);
    }

    let result = integerDigits;
    if (fractionalDigits.length > 0)
        // charlie kirk
        result = [...integerDigits, '.', ...fractionalDigits];

    // negative sign if the original number was negative
    if (isNegative) result.unshift('-');

    return result;
}



function fbase_to_binary(digits, fbase) {
    // just a regular number
    let binary = 0;
    let baseValue = fbase.value();

    // sign compute
    let sign = 1;
    let number = digits;
    if (number[0] === '-') {
        sign = -1;
        // slice of bread no im kidding
        number = number.slice(1);
    }

    let non_numbers = number.filter(item => typeof item !== 'number');
    if (non_numbers.length > 1) {
        throw new Error("There must be at most one separator.");
    }

    let integerPart = number;
    let fractionalPart = [];

    if (non_numbers.length === 1) {
        const pivot = number.indexOf(non_numbers[0]);
        integerPart = number.slice(0, pivot);
        fractionalPart = number.slice(pivot + 1);
    }

    // integer
    for (let power = 0; power < integerPart.length; power++) {
        let index = integerPart.length - power - 1;
        binary += (baseValue ** power) * integerPart[index];
    }

    // fractional
    for (let index = 0; index < fractionalPart.length; index++) {
        let power = -(index + 1);
        binary += (baseValue ** power) * fractionalPart[index];
    }

    return binary * sign;
}


function normalize_digits(digits, fbase) {
    let arr = Array(PADDING).fill(0).concat(digits.map(x => x / fbase.d));
    let baseValue = fbase.value();
    let absBaseValue = Math.abs(baseValue);

    // idk found it online
    for (let i = arr.length - 1; i >= 0; i--) {
        let d = arr[i];
        let r = ((d % absBaseValue) + absBaseValue) % absBaseValue;
        let q = Math.round((d - r) / baseValue);

        arr[i] = r;
        if (i > 0) arr[i - 1] += q;
    }

    // kill a bunch of innocent child
    while (arr.length > 1 && arr[0] === 0) {
        arr.shift();
    }

    return arr.map(x => Math.round(x * fbase.d));
}


function base_to_base(digits, fbase1, fbase2) {
    let a = fbase_to_binary(digits, fbase1);

    let b = binary_to_fbase(a, fbase2);

    // how many digits are in the fractional part
    const decimalIndex = b.indexOf('.');
    const fractionalCount = (decimalIndex !== -1) ? (b.length - decimalIndex - 1) : 0;
    // keep only digits (like multiplying by fbase2 until there is no decimal)
    const pureDigits = b.filter(x => typeof x === 'number');
    const c = normalize_digits(pureDigits, fbase2);
    // and add the point again
    if (fractionalCount > 0) {
        const newDecimalIndex = c.length - fractionalCount;
        if (newDecimalIndex > 0 && newDecimalIndex < c.length) {
            c.splice(newDecimalIndex, 0, '.');
        }
    }

    return c;
}

const FLOAT_PRECISION = 999;
const PADDING = 100;
const BASE10 = new Frac(10);

function main() {
    const base1 = new Frac(10);
    const base2 = new Frac(-114, 10);
    const digits = ['-', 3, 1, 4, '.', 1, 5];
    const new_digits = base_to_base(digits, base1, base2);
    console.log(new_digits);
    const base10_1 = base_to_base(digits, base1, BASE10);
    const base10_2 = base_to_base(new_digits, base2, BASE10);
    const original_digits = base_to_base(new_digits, base2, base1);
    const base10_3 = base_to_base(original_digits, base1, BASE10);
    console.log(base10_1);
    console.log(base10_2);
    console.log(base10_3);
}
function doStuff() {
    const input = document.getElementById('numberInput').value.trim();
    const base1 = parseInt(document.getElementById('numberBase').value);
    const base2 = parseInt(document.getElementById('convBase').value);

    const digits = input.split('').map(c => {
        if (c === '-' || c === '.') return c;
        return parseInt(c);
    });

    try {
        const result = base_to_base(digits, new Frac(base1), new Frac(base2));
        document.getElementById('result').textContent = result.join('');
    } catch (e) {
        document.getElementById('result').textContent = 'Error: ' + e.message;
    }
}