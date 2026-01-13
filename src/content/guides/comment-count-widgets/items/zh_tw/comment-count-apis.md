根據您的需求以及您想從瀏覽器、伺服器還是使用API SDK獲取資料，有幾個端點可用於獲取計數。

## 公共評論計數

您可以使用上面的小工具或使用它們所使用的API來獲取公共評論計數。這些API自2019年以來保持不變，並且永遠不會改變。

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

這將返回如下結構：

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

`postfix`屬性始終包含在內。

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

這將返回如下結構：

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

`counts`物件僅為具有計數的頁面填充。`translations`映射始終存在，因為它用於小工具。

### 公共端點行為 / 快取

公共端點有60秒的快取機制來處理流量高峰。在內部，這是伺服器記憶體中每個執行緒的LRU快取，因此當人們留下大量評論時，您可能會看到計數略有變化（上升然後暫時下降）。

公共端點始終返回*總*評論數，而不是根評論數。

### 伺服器端API / SDK

從伺服器獲取評論的方法是呼叫[Pages API](/guide-api.html#page-structure)並獲取頁面物件，其中包含總評論數和根評論數。我們提供SDK，允許您在不手動建構API請求的情況下呼叫此API，並提供類型化的返回值。
