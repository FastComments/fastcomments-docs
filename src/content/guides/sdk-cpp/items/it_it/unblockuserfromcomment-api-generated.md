## Parametri

| Name | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Sì |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Risposta

Restituisce: [`UnBlockCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnBlockCommentPublic_200_response.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di unBlockUserFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
auto paramsPtr = std::make_shared<UnBlockFromCommentParams>();
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId = boost::none;
api->unBlockUserFromComment(tenantId, commentId, *paramsPtr, userId, anonUserId)
    .then([](std::shared_ptr<UnBlockCommentPublic_200_response> resp){
        (void)resp;
    })
    .wait();
[inline-code-end]

---