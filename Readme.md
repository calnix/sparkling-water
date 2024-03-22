# Readme

To compute the public key for a private key using the BLS12-381 elliptic curve, you would typically follow these steps:

Generate a private key: Start with a random scalar value sk, which is the private key. 
Ensure that sk is a valid scalar within the curve's parameters (usually a number between 1 and the curve's order minus 1).

Compute the corresponding public key: Use the private key to compute the public key using scalar multiplication with the curve generator.

> cargo run