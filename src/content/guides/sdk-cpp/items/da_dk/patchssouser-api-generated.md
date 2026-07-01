## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Ja |  |
| updateComments | bool | Nej |  |

## Svar

Returnerer: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## Eksempel

[inline-code-attrs-start title = 'patchSSOUser Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user-456");
UpdateAPISSOUserData updateData;
updateData.email = U("jane.doe@example.com");
updateData.displayName = U("Jane Doe");
boost::optional<bool> updateComments = true;

api->patchSSOUser(tenantId, userId, updateData, updateComments)
    .then([](pplx::task<std::shared_ptr<PatchSSOUserAPIResponse>> t){
        try{
            auto resp = t.get();
            (void)resp;
        }catch(const std::exception&){
        }
    });
[inline-code-end]

---