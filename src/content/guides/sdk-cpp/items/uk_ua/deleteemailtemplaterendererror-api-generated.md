## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| errorId | string | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Приклад

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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