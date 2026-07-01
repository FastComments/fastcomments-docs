## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |

## 响应

返回：[`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## 示例

[inline-code-attrs-start title = 'deleteV2PageReact 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("page-456");
auto reactId = utility::conversions::to_string_t("react-789");
boost::optional<utility::string_t> correlationId = utility::conversions::to_string_t("corr-001");

api->deleteV2PageReact(tenantId, urlId, reactId)
   .then([](pplx::task<std::shared_ptr<CreateV1PageReact>> t) {
        auto result = t.get();
    });
[inline-code-end]