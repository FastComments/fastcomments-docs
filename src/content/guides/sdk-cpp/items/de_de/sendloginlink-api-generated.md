## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| redirectURL | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'sendLoginLink Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> redirectURL = utility::string_t(U("https://app.example.com/welcome"));
api->sendLoginLink(tenantId, userId, redirectURL)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t)
    {
        try
        {
            auto resp = t.get();
            if (resp) return resp;
            return std::make_shared<FlagCommentPublic_200_response>();
        }
        catch (...)
        {
            return std::make_shared<FlagCommentPublic_200_response>();
        }
    });
[inline-code-end]

---