## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badgeId | string | 예 |  |
| userId | string | 아니요 |  |
| commentId | string | 아니요 |  |
| broadcastId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RemoveUserBadgeResponse.h)

## 예제

[inline-code-attrs-start title = 'putRemoveBadge 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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