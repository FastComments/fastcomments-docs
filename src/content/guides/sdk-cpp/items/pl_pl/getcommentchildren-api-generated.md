---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| commentId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentChildren'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-12345");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getCommentChildren(commentId, sso).then([](std::shared_ptr<ModerationAPIChildCommentsResponse> resp){
    auto result = resp ? std::make_shared<ModerationAPIChildCommentsResponse>(*resp)
                       : std::make_shared<ModerationAPIChildCommentsResponse>();
    return result;
});
[inline-code-end]

---