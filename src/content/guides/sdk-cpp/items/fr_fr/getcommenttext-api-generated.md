## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| editKey | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIGetCommentTextResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-7f4b2a");
boost::optional<utility::string_t> editKey(utility::conversions::to_string_t("edit-xyz-789"));
boost::optional<utility::string_t> sso(utility::conversions::to_string_t("user@example.com"));
api->getCommentText(tenantId, commentId, editKey, sso)
.then([](pplx::task<std::shared_ptr<PublicAPIGetCommentTextResponse>> t){
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<PublicAPIGetCommentTextResponse>();
    } catch (...) {
        auto empty = std::make_shared<PublicAPIGetCommentTextResponse>();
    }
});
[inline-code-end]

---