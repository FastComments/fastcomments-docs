## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'postRestoreDeletedComment 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-8b3f4a2d");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->postRestoreDeletedComment(commentId, sso).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
    try {
        std::shared_ptr<APIEmptyResponse> resp = t.get();
        if (!resp) resp = std::make_shared<APIEmptyResponse>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---