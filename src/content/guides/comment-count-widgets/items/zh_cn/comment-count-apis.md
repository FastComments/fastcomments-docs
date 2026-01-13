根据您的需求以及您想从浏览器、服务器还是使用API SDK获取数据，有几个端点可用于获取计数。

## 公共评论计数

您可以使用上面的小部件或使用它们所使用的API来获取公共评论计数。这些API自2019年以来保持不变，并且永远不会改变。

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

这将返回如下结构：

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

`postfix`属性始终包含在内。

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

这将返回如下结构：

[inline-code-attrs-start title = 'Bulk Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

`counts`对象仅为具有计数的页面填充。`translations`映射始终存在，因为它用于小部件。

### 公共端点行为 / 缓存

公共端点有60秒的缓存机制来处理流量高峰。在内部，这是服务器内存中每个线程的LRU缓存，因此当人们留下大量评论时，您可能会看到计数略有变化（上升然后暂时下降）。

公共端点始终返回*总*评论数，而不是根评论数。

### 服务器端API / SDK

从服务器获取评论的方法是调用[Pages API](/guide-api.html#page-structure)并获取页面对象，其中包含总评论数和根评论数。我们提供SDK，允许您在不手动构建API请求的情况下调用此API，并提供类型化的返回值。
