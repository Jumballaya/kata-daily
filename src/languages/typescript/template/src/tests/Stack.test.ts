import assert from 'node:assert';
import { describe, it } from 'node:test';

const day = parseInt(process.env.KATA_DAY || '1');
const { Stack } = require(`../day${day}/Stack`);

describe('Stack', () => {
    it('should stack', () => {
       const s = new Stack(); 
       s.push(0);
       s.push(1);
       s.push(2);
       s.push(3);
       assert.equal(s.length, 4);
       assert.equal(s.peek(), 3);
       assert.equal(s.pop(), 3);
       assert.equal(s.length, 3);
       assert.equal(s.peek(), 2);
       assert.equal(s.pop(), 2);
       assert.equal(s.length, 2);
       assert.equal(s.pop(), 1);
       assert.equal(s.pop(), 0);
       assert.equal(s.pop(), undefined);
       assert.equal(s.peek(), undefined);
    });
});
