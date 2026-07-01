## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| options | const PostSetCommentSpamStatusOptions& | 是 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'postSetCommentSpamStatus 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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