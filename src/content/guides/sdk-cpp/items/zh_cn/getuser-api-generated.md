## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`GetUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetUserResponse.h)

## 示例

[inline-code-attrs-start title = 'getUser 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user-789");
boost::optional<utility::string_t> optTag = boost::none;

api->getUser(tenantId, userId)
    .then([=](pplx::task<std::shared_ptr<GetUserResponse>> task) {
        try {
            auto response = task.get();
            if (!response) {
                response = std::make_shared<GetUserResponse>();
            }
            // 根据需要处理响应
        } catch (const std::exception&) {
            // 处理错误
        }
    });
[inline-code-end]

---