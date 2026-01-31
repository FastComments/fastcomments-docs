## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | bool | No |  |

## Response

Returns: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## Example

[inline-code-attrs-start title = 'putSSOUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto userId = utility::string_t(U("user@example.com"));
auto updateDataPtr = std::make_shared<UpdateAPISSOUserData>();
updateDataPtr->setEmail(userId);
updateDataPtr->setDisplayName(utility::string_t(U("Jane Doe")));
boost::optional<bool> updateComments = true;
api->putSSOUser(tenantId, userId, *updateDataPtr, updateComments).then([](std::shared_ptr<PutSSOUserAPIResponse> resp){
    if (resp) {
        std::cout << "SSO user updated successfully\n";
    } else {
        std::cout << "Failed to update SSO user\n";
    }
});
[inline-code-end]
