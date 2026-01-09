## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| domain | string | はい |  |

## レスポンス

戻り値: [`DeleteDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteDomainConfig_200_response.h)

## 例

[inline-code-attrs-start title = 'deleteDomainConfigの例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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