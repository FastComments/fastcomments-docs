## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Risposta

Restituisce: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di flagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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