## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getCommentChildren Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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