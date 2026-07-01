## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|------|------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentTextResponse.h)

## 例

[inline-code-attrs-start title = 'getModerationCommentText の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654");
boost::optional<utility::string_t> sso = U("sso-token-abc");

api->getModerationCommentText(tenantId, commentId, sso)
    .then([](pplx::task<std::shared_ptr<GetCommentTextResponse>> t) {
        try {
            auto resp = t.get();
            auto text = std::make_shared<std::string>(resp->commentText);
        } catch (const std::exception&) {
        }
    });
[inline-code-end]