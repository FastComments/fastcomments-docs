## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| banUserUndoParams | BanUserUndoParams | Yes |  |
| sso | string | No |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple postBanUserUndo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
BanUserUndoParams banParams;
banParams.userId = utility::string_t(U("user-567"));
banParams.reason = utility::string_t(U("reinstated"));
boost::optional<utility::string_t> sso = utility::string_t(U("sso-token-abc"));

api->postBanUserUndo(tenantId, banParams, sss).then([](std::shared_ptr<APIEmptyResponse> resp){
    // gérer le succès
});
[inline-code-end]