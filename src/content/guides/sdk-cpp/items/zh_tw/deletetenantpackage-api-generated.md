## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 範例

[inline-code-attrs-start title = 'deleteTenantPackage 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pkgId = U("package-456");
boost::optional<utility::string_t> ifMatch = boost::optional<utility::string_t>(U("\"etag-789\""));
auto fallback = std::make_shared<FlagCommentPublic_200_response>();
auto task = api->deleteTenantPackage(tenantId, pkgId).then([fallback](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) return resp;
        return fallback;
    } catch (const std::exception&) {
        return fallback;
    }
});
task.wait();
[inline-code-end]

---