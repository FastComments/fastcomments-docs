## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## 示例

[inline-code-attrs-start title = 'getCommentChildren 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-12345");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getCommentChildren(commentId, sso).then([](std::shared_ptr<ModerationAPIChildCommentsResponse> resp){
    auto result = resp ? std::make_shared<ModerationAPIChildCommentsResponse>(*resp)
                       : std::make_shared<ModerationAPIChildCommentsResponse>();
    return result;
});
[inline-code-end]

---