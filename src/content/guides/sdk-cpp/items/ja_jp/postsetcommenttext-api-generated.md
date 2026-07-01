## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| setCommentTextParams | SetCommentTextParams | Yes |  |
| options | const PostSetCommentTextOptions& | Yes |  |

## レスポンス

戻り値: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentTextResponse.h)

## 例

[inline-code-attrs-start title = 'postSetCommentText の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto commentId = U("comment-456");
SetCommentTextParams params;
params.text = U("Revised comment content");
PostSetCommentTextOptions options;
options.requestId = boost::optional<utility::string_t>(U("req-987"));
api->postSetCommentText(tenantId, commentId, params, options)
    .then([](std::shared_ptr<SetCommentTextResponse> resp) {
        auto updatedId = resp->commentId;
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]