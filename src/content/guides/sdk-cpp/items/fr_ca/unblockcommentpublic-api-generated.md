## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`UnBlockCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnBlockCommentPublic_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de unBlockCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto publicParams = std::make_shared<PublicBlockFromCommentParams>();
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t(U("user@example.com")));
api->unBlockCommentPublic(utility::string_t(U("my-tenant-123")), utility::string_t(U("comment-98765")), *publicParams, sso)
.then([](pplx::task<std::shared_ptr<UnBlockCommentPublic_200_response>> task) {
    try {
        auto resp = task.get();
        if (resp) std::cout << "Unblocked comment successfully" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Unblock failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---