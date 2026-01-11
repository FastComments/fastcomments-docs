## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPISSOUserData | CreateAPISSOUserData | Yes |  |

## Response

Returns: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddSSOUserAPIResponse.h)

## Example

[inline-code-attrs-start title = 'addSSOUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto payload = std::make_shared<CreateAPISSOUserData>();
payload->email = utility::string_t(U("jane.doe@example.com"));
payload->displayName = utility::string_t(U("Jane Doe"));
payload->ssoId = utility::string_t(U("jdoe-azure-12345"));
payload->roles = std::vector<utility::string_t>{ utility::string_t(U("moderator")) };
payload->externalId = boost::optional<utility::string_t>(utility::string_t(U("ext-9876")));
payload->sendInvite = boost::optional<bool>(true);

api->addSSOUser(utility::string_t(U("my-tenant-123")), *payload)
.then([](std::shared_ptr<AddSSOUserAPIResponse> resp){
    if (resp) std::cout << "Added SSO user successfully\n";
    else std::cout << "Failed to add SSO user\n";
});
[inline-code-end]
