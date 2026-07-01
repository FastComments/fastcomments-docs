## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUserResponse.h)

## 示例

[inline-code-attrs-start title = 'getTenantUser 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
api->getTenantUser(tenantId, userId)
    .then([](pplx::task<std::shared_ptr<GetTenantUserResponse>> task) {
        try {
            auto response = task.get();
            // 根据需要使用响应
        } catch (const std::exception&) {
            // 错误处理
        }
    });
[inline-code-end]