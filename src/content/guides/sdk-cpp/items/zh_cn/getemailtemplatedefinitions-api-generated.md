## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 响应

返回: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateDefinitionsResponse.h)

## 示例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> locale = boost::none;
api->getEmailTemplateDefinitions(tenantId)
.then([=](pplx::task<std::shared_ptr<GetEmailTemplateDefinitionsResponse>> task) {
    try {
        auto resp = task.get();
        auto safeResp = resp ? resp : std::make_shared<GetEmailTemplateDefinitionsResponse>();
        return safeResp;
    } catch (const std::exception&) {
        return std::make_shared<GetEmailTemplateDefinitionsResponse>();
    }
});
[inline-code-end]

---