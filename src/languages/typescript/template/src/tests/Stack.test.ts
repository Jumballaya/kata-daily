import assert from 'node:assert';
import { describe, it } from 'node:test';

const day = parseInt(process.env.KATA_DAY || '1');
const { Stack } = require(`../day${day}/Stack`);

describe('Stack', () => {
    it('should stack', () => {
       const s = new Stack(); 
       assert(false);
    });
});
