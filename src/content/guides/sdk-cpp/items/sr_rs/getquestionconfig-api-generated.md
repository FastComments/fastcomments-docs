## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetQuestionConfigResponse.h)

## Пример

[inline-code-attrs-start title = 'getQuestionConfig Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto questionId = utility::conversions::to_string_t("question-456");

api->getQuestionConfig(tenantId, questionId)
    .then([](pplx::task<std::shared_ptr<GetQuestionConfigResponse>> task) {
        try {
            auto response = task.get();
            // Use response as needed
        } catch (const std::exception&) {
            // Handle error
        }
    });
[inline-code-end]