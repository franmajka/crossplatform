import { Library } from 'ffi-napi';

export const { factorial, sum, getHello } = Library('../lib/target/release/liblab3', {
  factorial: ['uint64', ['uint64']],
  sum: ['double', ['float', 'double']],
  getHello: ['string', ['string']],
});
