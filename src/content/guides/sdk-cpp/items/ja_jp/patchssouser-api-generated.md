## パラメーター

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateAPISSOUserData | UpdateAPISSOUserData | はい |  |
| updateComments | bool | いいえ |  |

## レスポンス

戻り値: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## 例

[inline-code-attrs-start title = 'patchSSOUser の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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