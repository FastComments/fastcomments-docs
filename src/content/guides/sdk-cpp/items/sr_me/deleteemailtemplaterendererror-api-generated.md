## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| errorId | string | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за deleteEmailTemplateRenderError'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("template-456");
boost::optional<utility::string_t> errorIdOpt = U("err-98765");
api->deleteEmailTemplateRenderError(tenantId, templateId, (errorIdOpt ? *errorIdOpt : utility::string_t()))
.then([=](pplx::task<std::shared_ptr<APIEmptyResponse>> task)
{
    try
    {
        auto resp = task.get();
        auto result = resp ? resp : std::make_shared<APIEmptyResponse>();
    }
    catch (const std::exception &)
    {
        auto fallback = std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]

---