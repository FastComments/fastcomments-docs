## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantBody | UpdateTenantBody | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateTenant Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("550e8400-e29b-41d4-a716-446655440000");
UpdateTenantBody updateTenantBody;
updateTenantBody.displayName = utility::string_t(U("Acme Corp Comments"));
updateTenantBody.supportEmail = boost::optional<utility::string_t>(U("support@acme.com"));
updateTenantBody.enabled = boost::optional<bool>(true);
api->updateTenant(tenantId, id, updateTenantBody)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task){
    try {
        auto resp = task.get();
        auto safeResp = resp ? resp : std::make_shared<FlagCommentPublic_200_response>();
        (void)safeResp;
    } catch (const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]
