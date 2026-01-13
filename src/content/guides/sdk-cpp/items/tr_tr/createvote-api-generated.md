## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| direction | string | Evet |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |

## Yanıt

Döndürür: [`VoteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteComment_200_response.h)

## Örnek

[inline-code-attrs-start title = 'createVote Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t direction = U("up");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
api->createVote(tenantId, commentId, direction, userId, boost::none)
.then([](pplx::task<std::shared_ptr<VoteComment_200_response>> task) {
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<VoteComment_200_response>();
    } catch (...) {}
});
[inline-code-end]

---