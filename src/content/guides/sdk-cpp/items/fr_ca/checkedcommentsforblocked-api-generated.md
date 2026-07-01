## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentIds | string | Yes |  |
| sso | string | No |  |

## Réponse

Retourne : [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CheckBlockedCommentsResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple checkedCommentsForBlocked'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto commentIds = U("cmt-001,cmt-002");
boost::optional<utility::string_t> sso = U("user@example.com");

api->checkedCommentsForBlocked(tenantId, commentIds, sso).then([](std::shared_ptr<CheckBlockedCommentsResponse> resp){
    (void)resp;
});
[inline-code-end]