## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| sure | string | 否 |  |

## 响应

返回: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 示例

[inline-code-attrs-start title = 'deleteTenant 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("flag-456");
boost::optional<utility::string_t> sure = utility::string_t(U("true"));
api->deleteTenant(tenantId, id, sure)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            auto safeResp = resp ? resp : std::make_shared<FlagCommentPublic_200_response>();
            (void)safeResp;
        } catch (...) {}
    });
[inline-code-end]

---