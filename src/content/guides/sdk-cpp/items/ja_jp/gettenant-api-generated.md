## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetTenant_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenant_200_response.h)

## 例

[inline-code-attrs-start title = 'getTenant の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("admin@fastcomments.com");
boost::optional<utility::string_t> asOf = boost::optional<utility::string_t>(U("2026-01-01T00:00:00Z"));
api->getTenant(tenantId, id).then([asOf](pplx::task<std::shared_ptr<GetTenant_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) return;
        auto processed = std::make_shared<GetTenant_200_response>(*resp);
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---