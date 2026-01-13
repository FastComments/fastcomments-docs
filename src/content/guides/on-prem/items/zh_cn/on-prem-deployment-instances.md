### Required Components

对于本地部署 (On-Prem)，FastComments 仅由一个应用服务器和一个数据库组成。我们简化了部署，使应用可以直接为所有流量提供服务，而无需添加其他组件。

应用服务器以 Docker 镜像形式提供，可与任何容器管理解决方案一起部署。

数据库 MongoDB 可以自建，也可以由其他提供商托管，例如 AWS DocumentDB 或 MongoDB Atlas。

FastComments 目前在 MongoDB 7 上进行了测试，但我们旨在与 DocumentDB 兼容以简化部署。

### Instance Sizes

你会发现 FastComments 已经相当优化，应用本身通常不需要很大的机器就能保持较低的 P99 响应时间。

所有批处理和 cron 作业都使用流式处理以限制总体内存使用。

下面的应用服务器和数据库表可帮助进行容量评估。

### Application Server Instances


| Concurrent Users | Total Cluster CPUs | Total Cluster Memory |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

例如，单个 CPU 内核每秒处理约 100 个评论线程通常不会使用超过 250mb RSS。

### Database Server Instances

数据库的规模取决于工作集大小（即在任意时间点访问的数据量）以及并发请求数。

FastComments 对 Mongo 相当友好，对于热点查询它使用索引提示、流式游标，并在多个区域设置并发限制以防止下游系统过载。

下面是关于数据库实例大小的一般指导。**请注意，这里是__每个实例__的配置，而不是集群的总资源**。

| Concurrent Users | Comments Stored | CPUs Per Instance | Memory Per Instance |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

以上表格为保守估计。根据你的具体配置（页面大小、评论量等），实际需求可能有所不同。