## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| setCommentTextParams | SetCommentTextParams | 是 |  |
| options | const PostSetCommentTextOptions& | 是 |  |

## 回應

返回：[`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentTextResponse.h)

## 範例

[inline-code-attrs-start title = 'postSetCommentText 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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