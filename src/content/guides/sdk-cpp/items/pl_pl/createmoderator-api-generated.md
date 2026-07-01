## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| createModeratorBody | CreateModeratorBody | Yes |  |

## Odpowiedź

Zwraca: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateModeratorResponse.h)

## Przykład

[inline-code-attrs-start title = 'createModerator Przykład'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateModeratorBody moderatorBody;
moderatorBody.email = utility::conversions::to_string_t("moderator@example.com");
moderatorBody.name = utility::conversions::to_string_t("John Moderator");
moderatorBody.notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Community moderator"));
api->createModerator(utility::conversions::to_string_t("my-tenant-123"), moderatorBody)
    .then([](std::shared_ptr<CreateModeratorResponse> resp) {});
[inline-code-end]