## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateTenantPackage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-789");
UpdateTenantPackageBody body;
body.packageName = utility::string_t(U("Pro Moderation"));
body.seatCount = boost::optional<int>(250);
body.billingContact = boost::optional<utility::string_t>(U("billing@acme.com"));
api->updateTenantPackage(tenantId, packageId, body).then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    if (resp) {
        auto resultCopy = std::make_shared<FlagCommentPublic_200_response>(*resp);
    }
    return pplx::task_from_result();
});
[inline-code-end]
