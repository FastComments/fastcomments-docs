## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## Örnek

[inline-code-attrs-start title = 'getCommentChildren Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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