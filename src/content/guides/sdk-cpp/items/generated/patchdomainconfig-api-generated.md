## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'patchDomainConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("comments.example.com");
PatchDomainConfigParams patchParams;
patchParams.allowComments = boost::optional<bool>(true);
patchParams.moderationMode = boost::optional<utility::string_t>(U("pre-moderation"));
auto fallback = std::make_shared<GetDomainConfig_200_response>();
api->patchDomainConfig(tenantId, domainToUpdate, patchParams)
.then([fallback](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> t)
{
    try
    {
        auto resp = t.get();
        if(!resp) resp = fallback;
        (void)resp;
    }
    catch(const std::exception &)
    {
        auto resp = fallback;
        (void)resp;
    }
});
[inline-code-end]
