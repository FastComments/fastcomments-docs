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
boost::optional<utility::string_t> environment = boost::optional<utility::string_t>(U("production"));
api->getDomainConfigs(tenantId).then([environment](std::shared_ptr<GetDomainConfigs_200_response> resp) -> pplx::task<std::shared_ptr<GetDomainConfigs_200_response>> {
    if (!resp) return pplx::task_from_result<std::shared_ptr<GetDomainConfigs_200_response>>(nullptr);
    auto resultCopy = std::make_shared<GetDomainConfigs_200_response>(*resp);
    return pplx::task_from_result(resultCopy);
});
[inline-code-end]
