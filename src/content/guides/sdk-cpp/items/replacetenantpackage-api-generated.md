## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'replaceTenantPackage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-abc-789");
ReplaceTenantPackageBody body;
boost::optional<utility::string_t> operatorEmail = boost::optional<utility::string_t>(U("admin@example.com"));
api->replaceTenantPackage(tenantId, packageId, body)
.then([=](std::shared_ptr<FlagCommentPublic_200_response> resp)
{
    if (resp)
    {
        auto copy = std::make_shared<FlagCommentPublic_200_response>(*resp);
        (void)copy;
    }
});
[inline-code-end]
