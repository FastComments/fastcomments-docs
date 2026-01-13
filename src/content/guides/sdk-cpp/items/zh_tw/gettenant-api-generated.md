## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetTenant_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenant_200_response.h)

## 範例

[inline-code-attrs-start title = 'getTenant 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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