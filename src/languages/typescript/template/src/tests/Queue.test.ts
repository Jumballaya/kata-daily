import assert from 'node:assert';
import { describe, it } from 'node:test';

const day = parseInt(process.env.KATA_DAY || '1');
const { Queue } = require(`../day${day}/Stack`);


describe('Queue', () => {
    it('should queue', () => {
       const q = new Queue(); 
       assert(false);
    });
});
