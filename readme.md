```                                                                                            
RRRRRRRRRRRRRRRRR           IIIIIIIIII        BBBBBBBBBBBBBBBBB              SSSSSSSSSSSSSSS 
R::::::::::::::::R          I::::::::I        B::::::::::::::::B           SS:::::::::::::::S
R::::::RRRRRR:::::R         I::::::::I        B::::::BBBBBB:::::B         S:::::SSSSSS::::::S
RR:::::R     R:::::R        II::::::II        BB:::::B     B:::::B        S:::::S     SSSSSSS
  R::::R     R:::::R          I::::I            B::::B     B:::::B        S:::::S            
  R::::R     R:::::R          I::::I            B::::B     B:::::B        S:::::S            
  R::::RRRRRR:::::R           I::::I            B::::BBBBBB:::::B          S::::SSSS         
  R:::::::::::::RR            I::::I            B:::::::::::::BB            SS::::::SSSSS    
  R::::RRRRRR:::::R           I::::I            B::::BBBBBB:::::B             SSS::::::::SS  
  R::::R     R:::::R          I::::I            B::::B     B:::::B               SSSSSS::::S 
  R::::R     R:::::R          I::::I            B::::B     B:::::B                    S:::::S
  R::::R     R:::::R          I::::I            B::::B     B:::::B                    S:::::S
RR:::::R     R:::::R        II::::::II        BB:::::BBBBBB::::::B        SSSSSSS     S:::::S
R::::::R     R:::::R ...... I::::::::I ...... B:::::::::::::::::B  ...... S::::::SSSSSS:::::S
R::::::R     R:::::R .::::. I::::::::I .::::. B::::::::::::::::B   .::::. S:::::::::::::::SS 
RRRRRRRR     RRRRRRR ...... IIIIIIIIII ...... BBBBBBBBBBBBBBBBB    ......  SSSSSSSSSSSSSSS   
```                                                                                          

---

[![Rust](https://github.com/dejanfajfar/ribs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/dejanfajfar/ribs/actions/workflows/rust.yml)

---


> My personal attempt at a simple battle system


# Introduction

I like learning thing while doing something with them. So this is my attempt at learning RUST.

# Installation

No installatio required

## Docker images

There is a `Dockerfile` provided. It build the application and provides you with a docker image that you can thgen start and enjoy the application

```shell
?> docker build . -t ribs:latest
```

## Docker compose 

Having a docker image is great an all but if you want to run the application then you will need something more. And before you go out and install everthing I made a simple `docker-compose` file that starts everything for you.

```shell
?> docker compose up
```

The _API_ is exposed on port 7777.

---

# Usefull links

https://doc.rust-lang.org/beta/style-guide/index.html