## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | int32_t | 否 |  |

## 响应

返回: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUsersResponse.h)

## 示例

[inline-code-attrs-start title = 'getSSOUsers 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<int32_t> skip = 25;
api->getSSOUsers(tenantId, skip).then([](pplx::task<std::shared_ptr<GetSSOUsersResponse>> t) {
    try {
        auto response = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]