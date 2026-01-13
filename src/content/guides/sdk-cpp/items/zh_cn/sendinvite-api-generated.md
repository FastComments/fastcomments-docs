## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## 响应

返回: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 示例

[inline-code-attrs-start title = 'sendInvite 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("comment-456");
utility::string_t fromName = U("moderator@example.com");
boost::optional<utility::string_t> inviteNote = boost::optional<utility::string_t>(U("Please review this flagged comment"));
auto task = api->sendInvite(tenantId, id, fromName)
    .then([inviteNote](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) -> std::shared_ptr<FlagCommentPublic_200_response> {
        try {
            auto resp = t.get();
            auto wrapped = std::make_shared<FlagCommentPublic_200_response>(*resp);
            (void)inviteNote;
            return wrapped;
        } catch (...) {
            throw;
        }
    });
[inline-code-end]

---