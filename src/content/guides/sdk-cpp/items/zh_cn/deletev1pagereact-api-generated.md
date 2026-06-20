## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 响应

返回: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## 示例

[inline-code-attrs-start title = 'deleteV1PageReact 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-9876");
boost::optional<utility::string_t> requestId = U("req-456");
auto task = api->deleteV1PageReact(tenantId, urlId)
    .then([requestId](pplx::task<std::shared_ptr<CreateV1PageReact>> t) {
        try {
            auto result = t.get();
            if (!result) result = std::make_shared<CreateV1PageReact>();
            return result;
        } catch (const std::exception&) {
            return std::make_shared<CreateV1PageReact>();
        }
    });
task.wait();
[inline-code-end]