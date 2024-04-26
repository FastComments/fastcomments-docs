Like any distributed system FastComments needs some way to lock resources and procedures. These locks can be monitored via the `/locks-in-progress` endpoint.

[For example, here's the endpoint on our US shard](https://fastcomments.com/locks-in-progress).

This can be useful to see why the system is hung up, or under load. If perhaps an SRE wants to see why the system is experiencing high CPU load, they could
check this endpoint to get the name of the cron misbehaving.
