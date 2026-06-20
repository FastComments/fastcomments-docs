## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 响应

返回: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplatesResponse.h)

## 示例

[inline-code-attrs-start title = 'getEmailTemplates 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = boost::optional<double>(0.0);
api->getEmailTemplates(tenantId, skip)
.then([](std::shared_ptr<GetEmailTemplatesResponse> resp) -> std::shared_ptr<GetEmailTemplatesResponse> {
    auto finalResp = resp ? resp : std::make_shared<GetEmailTemplatesResponse>();
    return finalResp;
})
.wait();
[inline-code-end]

---