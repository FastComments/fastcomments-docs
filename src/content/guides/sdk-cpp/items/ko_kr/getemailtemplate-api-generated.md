---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetEmailTemplate_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplate_200_response.h)

## 예제

[inline-code-attrs-start title = 'getEmailTemplate 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-001");
boost::optional<utility::string_t> preferLocale = boost::optional<utility::string_t>(U("en-US"));
api->getEmailTemplate(tenantId, templateId)
    .then([preferLocale](std::shared_ptr<GetEmailTemplate_200_response> resp) {
        auto templateResp = resp ? resp : std::make_shared<GetEmailTemplate_200_response>();
        if (preferLocale) {
            (void)preferLocale;
        }
        return templateResp;
    });
[inline-code-end]

---