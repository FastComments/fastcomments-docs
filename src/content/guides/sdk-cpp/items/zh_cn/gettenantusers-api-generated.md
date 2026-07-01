## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 响应

返回: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUsersResponse.h)

## 示例

[inline-code-attrs-start title = 'getTenantUsers 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<double> skip = 10;

api->getTenantUsers(tenantId, skip).then([](pplx::task<std::shared_ptr<GetTenantUsersResponse>> t){
    try {
        auto resp = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]