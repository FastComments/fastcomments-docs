## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| direction | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## 示例

[inline-code-attrs-start title = 'postVote 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> direction = U("up");
boost::optional<utility::string_t> sso = U("sso-token-abc123");
api->postVote(commentId, direction, sso)
.then([](pplx::task<std::shared_ptr<VoteResponse>> task) {
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<VoteResponse>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---