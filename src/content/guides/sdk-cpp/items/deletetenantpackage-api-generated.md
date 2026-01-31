## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteTenantPackage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
utility::string_t packageId(U("pkg-456"));
boost::optional<utility::string_t> note(U("cleanup-unused-packages"));
api->deleteTenantPackage(tenantId, packageId)
    .then([note](std::shared_ptr<FlagCommentPublic_200_response> resp) {
        if (resp) {
            auto resultCopy = std::make_shared<FlagCommentPublic_200_response>(*resp);
        }
    });
[inline-code-end]
