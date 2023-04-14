import assert from 'node:assert';
import { describe, it } from 'node:test';

const day = parseInt(process.env.KATA_DAY || '1');
const { bubbleSort } = require(`../day${day}/BubbleSort`);

describe('BubbleSort', () => {

    it('can sort: worst case', () => {
        const arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        bubbleSort(arr);
        assert.equal(arr[0], 0);
        assert.equal(arr[10], 10);
    });

    it('can sort long list', () => {
        const arr = new Array(5000);
        for (let i = 0; i < 5000; i++) {
            arr[i] = Math.random() * 500000;
        }
        bubbleSort(arr);
        assert(arr[0] < arr[100]);
        assert(arr[50] < arr[500]);
        assert(arr[400] < arr[1000]);
        assert(arr[1400] < arr[3000]);
        assert(arr[2700] < arr[4999]);
    });
})
