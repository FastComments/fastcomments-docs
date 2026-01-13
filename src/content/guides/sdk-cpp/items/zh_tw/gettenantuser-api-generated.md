## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 回應

回傳: [`GetTenantUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUser_200_response.h)

## 範例

[inline-code-attrs-start title = 'getTenantUser 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("alice@example.com");
boost::optional<utility::string_t> includeMetadata = boost::optional<utility::string_t>(U("true"));
api->getTenantUser(tenantId, userId).then([includeMetadata](pplx::task<std::shared_ptr<GetTenantUser_200_response>> t)
{
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<GetTenantUser_200_response>();
        if (includeMetadata && result) { (void)includeMetadata; }
        (void)result;
    } catch (const std::exception& e) {
        auto fallback = std::make_shared<GetTenantUser_200_response>();
        (void)fallback;
    }
});
[inline-code-end]

---