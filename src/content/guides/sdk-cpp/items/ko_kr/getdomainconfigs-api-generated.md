## Parameters

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## Response

반환: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigsResponse.h)

## Example

[inline-code-attrs-start title = 'getDomainConfigs 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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