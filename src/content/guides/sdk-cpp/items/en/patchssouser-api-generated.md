## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | bool | No |  |

## Response

Returns: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## Example

[inline-code-attrs-start title = 'patchSSOUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
auto updateData = std::make_shared<UpdateAPISSOUserData>();
updateData->email = U("user@example.com");
updateData->displayName = U("Jane Doe");
boost::optional<bool> updateComments = true;
api->patchSSOUser(tenantId, userId, *updateData, updateComments)
.then([](pplx::task<std::shared_ptr<PatchSSOUserAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) std::cout << "Patched SSO user\n";
        else std::cout << "No response\n";
    } catch (const std::exception& e) {
        std::cerr << "Patch failed: " << e.what() << '\n';
    }
});
[inline-code-end]
