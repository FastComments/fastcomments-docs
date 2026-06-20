## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| spam | bool | 否 |  |
| permNotSpam | bool | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'postSetCommentSpamStatus 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-7890");
boost::optional<bool> spam = true;
boost::optional<bool> permNotSpam = false;
boost::optional<utility::string_t> sso = U("user@example.com");

api->postSetCommentSpamStatus(commentId, spam, permNotSpam, sso)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        auto ack = std::make_shared<APIEmptyResponse>();
        if (resp) *ack = *resp;
        return ack;
    } catch (...) {
        return std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]

---