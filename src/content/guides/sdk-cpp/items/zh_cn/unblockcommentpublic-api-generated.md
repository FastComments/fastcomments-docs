---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## 示例

[inline-code-attrs-start title = 'unBlockCommentPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-7890");
PublicBlockFromCommentParams params;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->unBlockCommentPublic(tenantId, commentId, params, sso)
.then([](std::shared_ptr<UnblockSuccess> res) {
    if (!res) res = std::make_shared<UnblockSuccess>();
    return res;
})
.then([](std::shared_ptr<UnblockSuccess> finalResult){
    (void)finalResult;
});
[inline-code-end]

---