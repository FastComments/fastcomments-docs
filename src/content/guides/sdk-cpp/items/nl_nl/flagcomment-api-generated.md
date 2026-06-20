## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Antwoord

Geeft terug: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'flagComment Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-98765");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId = boost::none;
api->flagComment(tenantId, commentId, userId, anonUserId)
    .then([](std::shared_ptr<FlagCommentResponse> resp) -> std::shared_ptr<FlagCommentResponse> {
        if (resp) return resp;
        return std::make_shared<FlagCommentResponse>();
    })
    .wait();
[inline-code-end]

---