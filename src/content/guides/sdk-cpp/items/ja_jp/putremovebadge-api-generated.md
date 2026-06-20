## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badgeId | string | はい |  |
| userId | string | いいえ |  |
| commentId | string | いいえ |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RemoveUserBadgeResponse.h)

## 例

[inline-code-attrs-start title = 'putRemoveBadge の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t badgeId = U("badge-12345");
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> commentId(U("cmt-4567"));
boost::optional<utility::string_t> broadcastId(U("broadcast-2022"));
boost::optional<utility::string_t> sso(U("sso-token-abc123"));
api->putRemoveBadge(badgeId, userId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<RemoveUserBadgeResponse>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<RemoveUserBadgeResponse>();
    } catch (const std::exception&) {}
});
[inline-code-end]

---