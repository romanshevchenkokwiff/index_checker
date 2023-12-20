const test = require('.');

declare var self: Worker;

self.onmessage = (event: MessageEvent) => {
  console.log(event.data);
  test.get_initial_params(event.data);
}
