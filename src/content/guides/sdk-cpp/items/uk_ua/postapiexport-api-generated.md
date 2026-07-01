## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| options | const PostApiExportOptions& | Так |  |

## Відповідь

Повертає: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportResponse.h)

## Приклад

[inline-code-attrs-start title = 'postApiExport Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
PostApiExportOptions options;
options.format = utility::string_t(U("json"));
options.email = utility::string_t(U("reports@example.com"));
options.startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-01T00:00:00Z")));
options.endDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-31T23:59:59Z")));

api->postApiExport(tenantId, options)
    .then([](std::shared_ptr<ModerationExportResponse> response) {
        if (response) {
            // обробка успішної відповіді експорту
        }
    })
    .wait();
[inline-code-end]