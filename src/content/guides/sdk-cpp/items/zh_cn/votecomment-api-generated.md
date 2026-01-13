## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 是 |  |
| voteBodyParams | VoteBodyParams | 是 |  |
| sessionId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`VoteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteComment_200_response.h)

## 示例

[inline-code-attrs-start title = 'voteComment 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
VoteBodyParams voteBodyParams;
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-7890");
utility::string_t urlId = U("/articles/2025/how-to-cpprest");
utility::string_t broadcastId = U("broadcast-321");
boost::optional<utility::string_t> sessionId = boost::optional<utility::string_t>(U("sess-0a1b2c"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-jwt-xyz"));

api->voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso)
    .then([](pplx::task<std::shared_ptr<VoteComment_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<VoteComment_200_response>();
        } catch (...) {
        }
    });
[inline-code-end]

---