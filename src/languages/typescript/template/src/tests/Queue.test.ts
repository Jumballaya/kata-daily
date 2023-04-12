import assert from 'node:assert';
import { describe, it } from 'node:test';

const day = parseInt(process.env.KATA_DAY || '1');
const { Queue } = require(`../day${day}/Queue`);

describe("Queue", () => {
    it("should queue", () => {
        const q = new Queue();
        q.enqueue(0);
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert.equal(q.length, 4);
        assert.equal(q.peek(), 0);
        assert.equal(q.deque(), 0);
        assert.equal(q.deque(), 1);
        assert.equal(q.peek(), 2);
        assert.equal(q.length, 2);
        assert.equal(q.deque(), 2);
        assert.equal(q.deque(), 3);
        assert.equal(q.peek(), undefined);
        assert.equal(q.deque(), undefined);

    });
});

