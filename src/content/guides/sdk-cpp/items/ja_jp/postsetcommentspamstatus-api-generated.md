## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| options | const PostSetCommentSpamStatusOptions& | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'postSetCommentSpamStatus の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = PostSetCommentSpamStatusOptions{};
options.isSpam = true;
options.reason = boost::optional<utility::string_t>{U"User reported spam"};

api->postSetCommentSpamStatus(U("my-tenant-123"), U("comment-789"), options)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        auto copy = std::make_shared<APIEmptyResponse>(*resp);
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch(const std::exception&) {}
    });
[inline-code-end]