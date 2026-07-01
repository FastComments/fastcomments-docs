## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sendEmail | string | No |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'deleteModerator 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto moderatorId = utility::conversions::to_string_t("mod-456");
boost::optional<utility::string_t> sendEmail = utility::conversions::to_string_t("admin@example.com");
api->deleteModerator(tenantId, moderatorId, sendEmail)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // 处理成功
    });
[inline-code-end]

---