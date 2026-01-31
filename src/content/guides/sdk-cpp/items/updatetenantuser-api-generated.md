## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantUserBody | UpdateTenantUserBody | Yes |  |
| updateComments | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateTenantUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto bodyPtr = std::make_shared<UpdateTenantUserBody>();
bodyPtr->email = utility::string_t(U("user@example.com"));
bodyPtr->displayName = utility::string_t(U("Jane Reviewer"));
boost::optional<utility::string_t> updateComments = boost::optional<utility::string_t>(utility::string_t(U("true")));
api->updateTenantUser(utility::string_t(U("my-tenant-123")), utility::string_t(U("user-98765")), *bodyPtr, updateComments)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    if (resp) {
        auto handled = resp;
        (void)handled;
    }
});
[inline-code-end]
