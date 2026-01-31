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
utility::string_t tenantId = utility::string_t("my-tenant-123");
utility::string_t id = utility::string_t("user@example.com");
auto updatePtr = std::make_shared<UpdateAPISSOUserData>();
updatePtr->email = utility::string_t("user@example.com");
updatePtr->displayName = utility::string_t("Jane Doe");
boost::optional<bool> updateComments = true;
api->patchSSOUser(tenantId, id, *updatePtr, updateComments)
  .then([](std::shared_ptr<PatchSSOUserAPIResponse> resp){
    if(!resp) return;
  });
[inline-code-end]
