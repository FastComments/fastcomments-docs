---
像任何分布式系统一样，FastComments 需要一种方式来锁定资源和流程。可以通过 `/locks-in-progress` 端点来监控这些锁。

[例如，这里是我们美国分片上的端点](https://fastcomments.com/locks-in-progress)。

这对于查看系统为何卡住或在高负载时很有用。如果 SRE 想弄清系统为何出现高 CPU 负载，他们可以
检查此端点以获取出问题的 cron 的名称。

---