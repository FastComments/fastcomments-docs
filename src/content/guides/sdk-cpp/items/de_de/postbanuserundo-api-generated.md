## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja | |
| banUserUndoParams | BanUserUndoParams | Ja | |
| sso | string | Nein | |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'postBanUserUndo Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
BanUserUndoParams banParams;
banParams.userId = utility::string_t(U("user-567"));
banParams.reason = utility::string_t(U("reinstated"));
boost::optional<utility::string_t> sso = utility::string_t(U("sso-token-abc"));

api->postBanUserUndo(tenantId, banParams, sso).then([](std::shared_ptr<APIEmptyResponse> resp){
    // Erfolg behandeln
});
[inline-code-end]