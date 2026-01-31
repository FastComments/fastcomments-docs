## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Returns: [`DeleteDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("example.com");
boost::optional<utility::string_t> ifMatch = U("W/\"etag-6789\"");
api->deleteDomainConfig(tenantId, domain)
.then([](pplx::task<std::shared_ptr<DeleteDomainConfig_200_response>> t){
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<DeleteDomainConfig_200_response>();
    } catch (const std::exception&){
    }
}).wait();
[inline-code-end]
