## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostSetCommentSpamStatusOptions& | Yes |  |

## 回應

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'postSetCommentSpamStatus 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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