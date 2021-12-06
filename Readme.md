# Rong

Ping is a simple command that is used to check the liveness of a server. Ping
measures the round-trip time for messages sent from the originating host to a
destination computer that are echoed back to the source. The name comes from
active sonar terminology that sends a pulse of sound and listens for the echo
to detect objects under water. This is a toy version of that program.


## Note

The idea for this toy implementation is that it should be easy to read and
implement so we will avoid implementing a lot features and just do the bare
minimum. One can go throught this readme and try to implement their own version
for it. Maybe even add more feaures along the way.

## Requirements

- Understanding of IP (The layer-3 protocol)


## Theory

Before delving into writing the toy clone for ping it is very important that we
understand the mechnaisms that we have in place to make the Network layer work. 
- First we have the [Internet Protocol](https://datatracker.ietf.org/doc/html/rfc791)
  - It deals with creation of IP datagrams
  - handles hop-by-hop delivery of these datagrams
- Another thing we have in place are Routing Tables
  - which contain entries on what path to take and are filled using algorithms
    (e.g. Link-State, etc)
- Lastly we have [ICMP](https://datatracker.ietf.org/doc/html/rfc792)
  - it helps with error reporting
  - it helps with diagnoses of problems

### What happens when we send the `ECHO_REQUEST` packet?
Looking at the Echo and Echo Reply section of the ICMP protocol specification
we can see a way that can help us develop what we set out to do.

- Hop-by-hop this packet will reach the destination
- The destination will read this message and reply with an `ECHO_REPLY`
- Hop-by-hop this reaches us back

## High level overview
- Create an `ECHO_REQUEST` packet
- Put this packet in an IP datagram
- Start a timer
- Send this packet to the destination
- Receive the `ECHO_REPLY` packet sent by the destination
- Stop the timer
- Calculate the elpased time


## Execution
`./run.sh --url=<ipv4 address of the destination>`

Note: In order to keep the code simple we don't do dns resolution so it expects
an ipv4 address and not a url.
