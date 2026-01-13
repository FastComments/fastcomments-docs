## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Da |  |
| updateComments | bool | Ne |  |

## Odgovor

Vrne: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer patchSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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