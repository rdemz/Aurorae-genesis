// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Auroraium {
    string public name = "Auroraium";
    string public symbol = "AURA";
    uint8 public decimals = 18;
    uint256 public totalSupply;

    address public founder;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);

    constructor(uint256 initialSupply, address founderAddress) {
        founder = founderAddress;
        uint256 founderShare = (initialSupply * 15) / 100;
        uint256 remaining = initialSupply - founderShare;

        balanceOf[founder] = founderShare;
        balanceOf[msg.sender] = remaining;
        totalSupply = initialSupply;

        emit Transfer(address(0), founder, founderShare);
        emit Transfer(address(0), msg.sender, remaining);
    }

    function transfer(address to, uint256 value) public returns (bool) {
        require(balanceOf[msg.sender] >= value, "Insufficient balance.");
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }
}
