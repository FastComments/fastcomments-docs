## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Oui |  |
| updateComments | bool | Non |  |

## Réponse

Retourne : [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple putSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateAPISSOUserData userData;
userData.email = utility::conversions::to_string_t("alice@example.com");
userData.first_name = utility::conversions::to_string_t("Alice");
userData.last_name = utility::conversions::to_string_t("Smith");
userData.role = utility::conversions::to_string_t("moderator");

api->putSSOUser(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("alice.smith"),
    userData,
    boost::optional<bool>(true)
).then([](pplx::task<std::shared_ptr<PutSSOUserAPIResponse>> t) {
    auto response = t.get();
});
[inline-code-end]