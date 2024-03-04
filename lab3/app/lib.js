import { Library } from 'ffi-napi';

export const { factorial } = Library('../lib/target/release/liblab3', {
  factorial: ['uint64', ['uint64']],
});
