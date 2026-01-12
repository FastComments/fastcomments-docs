## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetDomainConfigs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigs_200_response.h)

## Example

[inline-code-attrs-start title = 'getDomainConfigs Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> includeArchived = boost::optional<utility::string_t>(U("false"));
api->getDomainConfigs(tenantId).then([=](std::shared_ptr<GetDomainConfigs_200_response> resp) {
    auto cfg = resp ? resp : std::make_shared<GetDomainConfigs_200_response>();
    return cfg;
}).then([](std::shared_ptr<GetDomainConfigs_200_response> finalCfg) {
    (void)finalCfg;
});
[inline-code-end]
