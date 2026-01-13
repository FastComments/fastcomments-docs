## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| sendEmail | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'deleteModerator Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t moderatorId = U("moderator-456");
boost::optional<utility::string_t> sendEmail = boost::optional<utility::string_t>(U("moderator@notifications.example.com"));
api->deleteModerator(tenantId, moderatorId, sendEmail)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<FlagCommentPublic_200_response>();
            return resp;
        } catch (const std::exception&) {
            return std::make_shared<FlagCommentPublic_200_response>();
        }
    });
[inline-code-end]

---