## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| email | string | 是 |  |

## 回應

回傳：[`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByEmailAPIResponse.h)

## 範例

[inline-code-attrs-start title = 'getSSOUserByEmail 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t email = U("user@example.com");
boost::optional<utility::string_t> includeInactive = boost::optional<utility::string_t>(U("false"));
api->getSSOUserByEmail(tenantId, email).then([includeInactive](pplx::task<std::shared_ptr<GetSSOUserByEmailAPIResponse>> t) {
    try {
        auto resp = t.get();
        return resp ? resp : std::make_shared<GetSSOUserByEmailAPIResponse>();
    } catch (...) {
        return std::make_shared<GetSSOUserByEmailAPIResponse>();
    }
}).then([](std::shared_ptr<GetSSOUserByEmailAPIResponse> finalResp) {
    (void)finalResp;
});
[inline-code-end]

---