## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| sso | string | No |  |

## 响应

返回: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PostRemoveCommentResponse.h)

## 示例

[inline-code-attrs-start title = 'postRemoveComment 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->postRemoveComment(commentId, sso)
    .then([](pplx::task<std::shared_ptr<PostRemoveCommentResponse>> t) {
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<PostRemoveCommentResponse>();
            if (result) std::cout << "Comment removed successfully\n";
        } catch (const std::exception &e) {
            std::cerr << "Remove failed: " << e.what() << "\n";
        }
    });
[inline-code-end]

---