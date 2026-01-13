## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| skip | double | 否 |  |

## 响应

返回: [`GetEmailTemplateRenderErrors_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrors_200_response.h)

## 示例

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("email-template-789");
boost::optional<double> skip{10.0};
auto defaultResp = std::make_shared<GetEmailTemplateRenderErrors_200_response>();
api->getEmailTemplateRenderErrors(tenantId, id, skip).then([defaultResp](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrors_200_response>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = defaultResp;
        std::wcout << (resp ? U("Received render errors response\n") : U("No response\n"));
    } catch(...) {
        std::wcout << U("Failed to retrieve render errors\n");
    }
});
[inline-code-end]

---