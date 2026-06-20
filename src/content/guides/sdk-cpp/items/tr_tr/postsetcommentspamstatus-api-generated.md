## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| spam | bool | Hayır |  |
| permNotSpam | bool | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Örnek

[inline-code-attrs-start title = 'postSetCommentSpamStatus Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-7890");
boost::optional<bool> spam = true;
boost::optional<bool> permNotSpam = false;
boost::optional<utility::string_t> sso = U("user@example.com");

api->postSetCommentSpamStatus(commentId, spam, permNotSpam, sso)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        auto ack = std::make_shared<APIEmptyResponse>();
        if (resp) *ack = *resp;
        return ack;
    } catch (...) {
        return std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]