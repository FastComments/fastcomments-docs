## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| redirectURL | string | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'sendLoginLink 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> redirectUrl = boost::optional<utility::string_t>(U("https://app.example.com/welcome"));
api->sendLoginLink(tenantId, userId, redirectUrl).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        auto finalResp = resp ? resp : std::make_shared<APIEmptyResponse>();
        (void)finalResp;
    } catch (...) {
        auto fallback = std::make_shared<APIEmptyResponse>();
        (void)fallback;
    }
});
[inline-code-end]

---