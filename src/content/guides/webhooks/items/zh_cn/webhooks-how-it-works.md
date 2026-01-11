系统中对 Comment 对象的所有更改都会触发一个事件，该事件最终会进入队列。

初始 webhook 事件通常会在事件源发生后的六秒内发送。

如果您的 API 宕机，您可以在 Webhooks 管理页面监控此队列。

如果对您的 API 的请求失败，我们会按照一个计划将其重新排入队列。

该计划为 `1 Minute * the retry count`。如果调用失败一次，它将在一分钟后重试。如果失败两次，则会等待两分钟，依此类推。这样做是为了避免在因为负载相关原因导致您的 API 宕机时对其造成过载。

可以在[日志页面](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs)取消 Webhooks。