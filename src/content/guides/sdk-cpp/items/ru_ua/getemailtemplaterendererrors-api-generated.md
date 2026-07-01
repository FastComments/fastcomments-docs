## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| skip | double | Ні |  |

## Відповідь

Returns: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrorsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getEmailTemplateRenderErrors'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("email-template-789");
boost::optional<double> skip = 20.0;

api->getEmailTemplateRenderErrors(tenantId, id, skip)
    .then([](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrorsResponse>> task) {
        try {
            auto response = task.get();
            // Використайте відповідь за потреби
        } catch (const std::exception& ex) {
            // Обробити помилку
        }
    });
[inline-code-end]