## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| direction | string | Так |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |

## Відповідь

Повертає: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## Приклад

[inline-code-attrs-start title = 'createVote приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> userId(U("alice@example.com"));
boost::optional<utility::string_t> anonUserId;
api->createVote(U("my-tenant-123"), U("cmt-456"), U("upvote"), userId, anonUserId)
.then([](pplx::task<std::shared_ptr<VoteResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<VoteResponse>();
    } catch (const std::exception&) {
        auto fallback = std::make_shared<VoteResponse>();
    }
});
[inline-code-end]