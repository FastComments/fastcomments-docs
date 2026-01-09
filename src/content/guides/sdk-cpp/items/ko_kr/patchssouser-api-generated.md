---
## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateAPISSOUserData | UpdateAPISSOUserData | 예 |  |
| updateComments | bool | 아니오 |  |

## 응답

반환: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## 예제

[inline-code-attrs-start title = 'patchSSOUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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