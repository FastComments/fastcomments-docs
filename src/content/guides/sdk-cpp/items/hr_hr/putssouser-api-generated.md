## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | bool | No |  |

## Odgovor

Vraća: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## Primjer

[inline-code-attrs-start title = 'putSSOUser Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---