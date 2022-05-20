import log from 'loglevel';
import Module from './module.js';
import options from './options.js';

log.setLevel(options.logLevel || 'info');

const myModule = new Module(options);
myModule.run();
