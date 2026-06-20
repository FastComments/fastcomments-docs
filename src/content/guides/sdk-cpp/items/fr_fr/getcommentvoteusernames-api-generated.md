## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| dir | int32_t | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentVoteUserNames'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
int32_t dir = 1;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getCommentVoteUserNames(tenantId, commentId, dir, sso)
.then([](std::shared_ptr<GetCommentVoteUserNamesSuccessResponse> resp){
    auto result = resp ? resp : std::make_shared<GetCommentVoteUserNamesSuccessResponse>();
    std::cout << "Fetched comment vote user names" << std::endl;
});
[inline-code-end]

---