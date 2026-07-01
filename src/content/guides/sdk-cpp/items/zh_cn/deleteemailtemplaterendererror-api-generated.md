## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| errorId | string | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> optTemplateId = utility::conversions::to_string_t("template-456");
api->deleteEmailTemplateRenderError(
    utility::conversions::to_string_t("my-tenant-123"),
    *optTemplateId,
    utility::conversions::to_string_t("error-789"))
    .then([](std::shared_ptr<APIEmptyResponse>) {})
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (...) {}
    });
[inline-code-end]