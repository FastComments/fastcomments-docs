### Required Components
For most deployments, FastComments just consists of an application server, a database, and a live connection manager.

The live connection manger can be scaled horizontally if needed in conjunction with Redis.

### Deployment

With FastComments On Prem, there are two main options for deployment.

1. Single Docker Container (single machine).
2. Horizontal deployment.

### Single Docker Container

A Docker container can be provided that includes the entire software stack. On commodity hardware,
this will still perform well for tens of thousands of concurrent users.

#### Setup Cost

This has the lowest setup cost, since the machine running the container simply has to have Docker setup.


### Horizontal Deployment

For larger deployments, our technical consulting staff can set up an On Prem deployment of our software
that has more redundancy.

#### Setup Cost

This has the highest setup cost, since multiple machines have to be setup, secured, and monitored.
