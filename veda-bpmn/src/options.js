import fs from 'fs';

let cfg;

try {
  cfg = fs.readFileSync('./conf/veda-bpmn.properties', 'utf-8');
} catch (err) {
  console.log('config file "./conf/veda-bpmn.properties" not found');
}

try {
  cfg = fs.readFileSync('../conf/veda-bpmn.properties', 'utf-8');
} catch (err) {
  console.log('config file "../conf/veda-bpmn.properties" not found');
}

if (!cfg) {
  console.log('no config found, exit');
  process.exit(1);
}

const options = {};
cfg.replace(/^(?!\s*#\s*)(.*?)\s*=\s*(.*?)\s*$/gm, (_, k, v) => options[k] = v);

export default options;
