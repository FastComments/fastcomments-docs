### Required Components

For On-Prem, FastComments consists of an application server and a database. We have simplified the deployment so that
the application can serve all traffic directly without adding other components.

The application server is provided in a Docker image and can be deployed with any container management solution.

The database, MongoDB, can be self-hosted or hosted by another provider like AWS DocumentDB or MongoDB Atlas.

FastComments is currently tested with MongoDB 7, however we aim to be DocumentDB compatible to ease deployment.

### Instance Sizes

You will find that FastComments is fairly well optimized and doesn't require large machines for the application itself to keep low P99s.

All batch and cron jobs use streaming to limit total memory usage.

The tables below for the application server and database can assist with sizing.

### Application Server Instances


| Concurrent Users | Total Cluster CPUs | Total Cluster Memory |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

For example, a single core serving around 100 comment threads per second usually never uses more than 250mb RSS.

### Database Server Instances

Sizing the database depends on working set size, which is the amount of data you access at a given point in time, as well as concurrent requests.

FastComments is fairly kind to MongoDB, in that for the hot queries it uses index hints, streaming cursors, and has concurrency limits in various areas
to prevent overloading of downstream systems.

The following is a general guideline on database instance sizes. **Note that this is __per instance__, not total resources in the cluster**.

| Concurrent Users | Comments Stored | CPUs Per Instance | Memory Per Instance |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

The above tables are conservative estimates. You may find actual requirements differ based on your specific configuration (page sizes, comment volume, etc).