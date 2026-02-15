## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## Response

Returns: [`CreateTenantUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantUser_200_response.h)

## Example

[inline-code-attrs-start title = 'createTenantUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto bodyPtr = std::make_shared<CreateTenantUserBody>();
bodyPtr->email = utility::string_t(U("new.user@example.com"));
bodyPtr->name = utility::string_t(U("Jane Doe"));
bodyPtr->sendInvite = boost::optional<bool>(true);
api->createTenantUser(tenantId, *bodyPtr).then([](std::shared_ptr<CreateTenantUser_200_response> resp){
    (void)resp;
});
[inline-code-end]
