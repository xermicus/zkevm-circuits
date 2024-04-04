// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

/**
 * @title Fibonacci
 * @dev Calculate fibonacci numbers iteratively
 */
contract Fibonacci {
    function fibonacci(uint n) external pure returns (uint b) {
        if (n == 0) {
            return 0;
        }
        uint a = 1;
        b = 1;
        for (uint i = 2; i < n; i++) {
            uint c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}
