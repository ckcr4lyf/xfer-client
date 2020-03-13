# xfer-client
Rust based TCP file transfer - Client

A very simple program that takes a host to connect to, and a local file.
A connection is made to the host specified (simple TCP socket).

The local file is chunked and sent to the host over the TCP connection.

The TCP connection is then closed and the file transfer is "complete".

## Warning!

This is currently just a test project, and is missing major fetures such as:

* Sending metadata such as filesize beforehand
* Some sort of hash check to ensure the file was received
* Encryption for file transfers