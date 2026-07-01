## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Yes |  |

## 回應

回傳：[`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkCreateHashTagsResponse.h)

## 範例

[inline-code-attrs-start title = 'addHashTagsBulk 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
BulkCreateHashTagsBody bulkBody;
bulkBody.tags = { utility::string_t(U("announcement")), utility::string_t(U("feature-request")) };
bulkBody.description = boost::optional<utility::string_t>(U("Bulk tag creation"));
api->addHashTagsBulk(tenantId, bulkBody).then([](pplx::task<std::shared_ptr<BulkCreateHashTagsResponse>> task){
    try{
        auto resp = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]