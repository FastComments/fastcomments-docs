## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Returns: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'getDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantOverride;
utility::string_t tenantId = tenantOverride.value_or(utility::conversions::to_string_t("my-tenant-123"));
utility::string_t domain = utility::conversions::to_string_t("comments.mycompany.com");
api->getDomainConfig(tenantId, domain).then([](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> t){
    try {
        auto resp = t.get();
        auto config = resp ? resp : std::make_shared<GetDomainConfig_200_response>();
        (void)config;
    } catch (const std::exception& e) {
        auto fallback = std::make_shared<GetDomainConfig_200_response>();
        (void)fallback;
    }
});
[inline-code-end]
