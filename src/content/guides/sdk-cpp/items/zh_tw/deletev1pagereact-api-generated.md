## 參數

| 名稱 | 類型 | 必須 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## 回應

返回: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## 範例

[inline-code-attrs-start title = 'deleteV1PageReact 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("page-abc-456");
api->deleteV1PageReact(tenantId, urlId).then([](pplx::task<std::shared_ptr<CreateV1PageReact>> task){
    try{
        auto response = task.get();
        boost::optional<std::shared_ptr<CreateV1PageReact>> optResponse = response;
    }catch(...){
    }
});
[inline-code-end]