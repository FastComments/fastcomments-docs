---
## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateAPISSOUserData | UpdateAPISSOUserData | 是 |  |
| updateComments | bool | 否 |  |

## 回應

回傳： [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## 範例

[inline-code-attrs-start title = 'patchSSOUser 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateAPISSOUserData updateData;
updateData.email = utility::string_t(U"user@example.com");
updateData.displayName = utility::string_t(U"Jane Doe");
boost::optional<bool> updateComments = true;
auto responseHolder = std::make_shared<PatchSSOUserAPIResponse>();
api->patchSSOUser(utility::string_t(U"my-tenant-123"), utility::string_t(U"user@example.com"), updateData, updateComments)
.then([responseHolder](std::shared_ptr<PatchSSOUserAPIResponse> resp){
    if (resp) *responseHolder = *resp;
    return responseHolder;
});
[inline-code-end]

---