## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| domain | string | 是 |  |

## 回應

回傳: [`DeleteDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteDomainConfig_200_response.h)

## 範例

[inline-code-attrs-start title = 'deleteDomainConfig 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("comments.example.com");
boost::optional<utility::string_t> ifMatch = boost::optional<utility::string_t>(U("W/\"abc123\""));
api->deleteDomainConfig(tenantId, domain)
.then([](pplx::task<std::shared_ptr<DeleteDomainConfig_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto resultCopy = std::make_shared<DeleteDomainConfig_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---