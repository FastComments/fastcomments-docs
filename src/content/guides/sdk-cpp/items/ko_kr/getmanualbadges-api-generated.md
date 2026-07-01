## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## 응답

Returns: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantManualBadgesResponse.h)

## Example

[inline-code-attrs-start title = 'getManualBadges 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::make_optional(U("user@example.com"));

api->getManualBadges(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<GetTenantManualBadgesResponse>> t) {
        try {
            auto response = t.get();
            // 응답을 처리, 예: response->badgeList
        } catch (const std::exception& ex) {
            // 오류 처리
        }
    });
[inline-code-end]

---