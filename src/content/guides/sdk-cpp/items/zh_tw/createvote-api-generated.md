## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| direction | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## 範例

[inline-code-attrs-start title = 'createVote 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---