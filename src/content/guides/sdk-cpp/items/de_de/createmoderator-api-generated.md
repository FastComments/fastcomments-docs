---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| createModeratorBody | CreateModeratorBody | Ja |  |

## Antwort

Rückgabe: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateModeratorResponse.h)

## Beispiel

[inline-code-attrs-start title = 'createModerator Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateModeratorBody moderatorBody;
moderatorBody.email = utility::conversions::to_string_t("moderator@example.com");
moderatorBody.name = utility::conversions::to_string_t("John Moderator");
moderatorBody.notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Community moderator"));
api->createModerator(utility::conversions::to_string_t("my-tenant-123"), moderatorBody)
    .then([](std::shared_ptr<CreateModeratorResponse> resp) {});
[inline-code-end]

---