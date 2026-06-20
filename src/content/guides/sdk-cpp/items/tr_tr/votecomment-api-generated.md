## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| urlId | string | Evet |  |
| broadcastId | string | Evet |  |
| voteBodyParams | VoteBodyParams | Evet |  |
| sessionId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## Örnek

[inline-code-attrs-start title = 'voteComment Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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