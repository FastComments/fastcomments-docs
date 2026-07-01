## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | Yes |  |

## 回應

返回：[`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigsResponse.h)

## 範例

[inline-code-attrs-start title = 'getDomainConfigs 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> optionalTenant = tenantId;
api->getDomainConfigs(optionalTenant.value())
    .then([](std::shared_ptr<GetDomainConfigsResponse> response) {
        auto domains = response->getDomainList();
        for (const auto& d : domains) {
            std::cout << d << std::endl;
        }
    });
[inline-code-end]

---