## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| redirectURL | string | 否 |  |

## 响应

返回: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 示例

[inline-code-attrs-start title = 'sendLoginLink 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> redirectURL = utility::string_t(U("https://app.example.com/welcome"));
api->sendLoginLink(tenantId, userId, redirectURL)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t)
    {
        try
        {
            auto resp = t.get();
            if (resp) return resp;
            return std::make_shared<FlagCommentPublic_200_response>();
        }
        catch (...)
        {
            return std::make_shared<FlagCommentPublic_200_response>();
        }
    });
[inline-code-end]

---