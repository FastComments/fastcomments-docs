## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetBannedUsersFromCommentResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getBanUsersFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("comment-456"));
boost::optional<utility::string_t> sso = boost::make_optional(utility::string_t(U("sso-token-abc")));

api->getBanUsersFromComment(tenantId, commentId, sso).then([](pplx::task<std::shared_ptr<GetBannedUsersFromCommentResponse>> task) {
    try {
        auto response = task.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]