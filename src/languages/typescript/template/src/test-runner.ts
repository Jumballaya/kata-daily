
const day = parseInt(process.env.KATA_DAY || '1');
require(`./day${day}/tests/Queue.test`);
require(`./day${day}/tests/Stack.test`);
