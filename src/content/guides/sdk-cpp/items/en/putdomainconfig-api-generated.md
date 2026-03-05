## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'putDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("comments.example.com");
UpdateDomainConfigParams updateParams;
updateParams.enabled = true;
updateParams.requireModeration = boost::optional<bool>(true);
updateParams.allowedOrigins = boost::optional<std::vector<utility::string_t>>({U("https://www.example.com"), U("https://app.example.com")});
updateParams.notificationEmail = boost::optional<utility::string_t>(U("moderator@example.com"));
auto task = api->putDomainConfig(tenantId, domainToUpdate, updateParams)
    .then([](std::shared_ptr<GetDomainConfig_200_response> resp){
        auto updated = std::make_shared<GetDomainConfig_200_response>(*resp);
        return updated;
    });
task.wait();
[inline-code-end]
