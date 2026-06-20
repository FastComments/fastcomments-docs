---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | 是 |  |
| sso | string | 否 |  |

## 响应

返回： [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## 示例

[inline-code-attrs-start title = 'postCommentsByIds 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CommentsByIdsParams params;
params.tenantId = utility::string_t(U("my-tenant-123"));
params.commentIds = std::vector<utility::string_t>{ U("cmt-1001"), U("cmt-1002") };
boost::optional<utility::string_t> sso(U("user@example.com"));
api->postCommentsByIds(params, sso).then([](pplx::task<std::shared_ptr<ModerationAPIChildCommentsResponse>> t) {
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<ModerationAPIChildCommentsResponse>();
        std::cout << "Fetched child comments response: " << (result ? "present" : "empty") << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "Request failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---