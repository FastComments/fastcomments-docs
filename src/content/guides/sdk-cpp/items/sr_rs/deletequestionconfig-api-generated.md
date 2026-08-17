## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'deleteQuestionConfig Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto configId = utility::conversions::to_string_t("question-config-456");

api->deleteQuestionConfig(tenantId, configId)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // обради успешно брисање
    })
    .then([](pplx::task<void> t) {
        try {
            t.get();
        } catch (const std::exception&) {
            // обради грешку
        }
    });
[inline-code-end]