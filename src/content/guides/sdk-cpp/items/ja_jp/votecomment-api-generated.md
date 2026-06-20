## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| urlId | string | はい |  |
| broadcastId | string | はい |  |
| voteBodyParams | VoteBodyParams | はい |  |
| sessionId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## 例

[inline-code-attrs-start title = 'voteComment の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentId = utility::conversions::to_string_t("comment-456");
utility::string_t urlId = utility::conversions::to_string_t("/articles/how-to-cpp");
utility::string_t broadcastId = utility::conversions::to_string_t("broadcast-001");
VoteBodyParams voteBodyParams;
boost::optional<utility::string_t> sessionId = boost::optional<utility::string_t>(utility::conversions::to_string_t("session-abc-123"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"));
api->voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso)
.then([](std::shared_ptr<VoteResponse> resp){
    auto safeResp = resp ? resp : std::make_shared<VoteResponse>();
    return safeResp;
});
[inline-code-end]

---