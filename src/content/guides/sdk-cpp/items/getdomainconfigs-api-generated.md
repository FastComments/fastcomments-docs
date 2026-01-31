## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetDomainConfigs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigs_200_response.h)

## Example

[inline-code-attrs-start title = 'getDomainConfigs Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantIdOpt{ utility::string_t(U("my-tenant-123")) };
api->getDomainConfigs(tenantIdOpt.value())
.then([](std::shared_ptr<GetDomainConfigs_200_response> resp){
    auto cfg = std::make_shared<GetDomainConfigs_200_response>(*resp);
    std::cout << "Fetched domain configs for tenant" << std::endl;
    return cfg;
})
.wait();
[inline-code-end]
