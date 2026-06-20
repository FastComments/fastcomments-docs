## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwoord

Retourneert: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getComment Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> include = boost::optional<utility::string_t>(U("author,replies"));

api->getComment(tenantId, commentId).then([include](std::shared_ptr<APIGetCommentResponse> resp) {
    if (resp) {
        auto copy = std::make_shared<APIGetCommentResponse>(*resp);
        std::cout << "Comment retrieved for tenant" << std::endl;
    } else {
        std::cout << "Comment not found" << std::endl;
    }
});
[inline-code-end]

---