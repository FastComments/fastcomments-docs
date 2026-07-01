## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetTrustFactorOptions& | 是 |  |

## 回應

Returns: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserTrustFactorResponse.h)

## 範例

[inline-code-attrs-start title = 'getTrustFactor 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
GetTrustFactorOptions options;
options.userEmail = boost::optional<utility::string_t>(U("user@example.com"));
options.ipAddress = boost::optional<utility::string_t>(U("203.0.113.42"));
api->getTrustFactor(tenantId, options).then([](std::shared_ptr<GetUserTrustFactorResponse> resp) {
    if (resp) {
        std::cout << "Trust factor: " << resp->trustFactor << std::endl;
    }
});
[inline-code-end]