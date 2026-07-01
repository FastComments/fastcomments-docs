## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeCommentPinStatusResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de unPinComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto task = api->unPinComment(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("cmt-456789"),
    utility::conversions::to_string_t("broadcast-001"),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"))
);
task.then([](std::shared_ptr<ChangeCommentPinStatusResponse> resp){
    auto result = std::make_shared<ChangeCommentPinStatusResponse>(*resp);
});
[inline-code-end]