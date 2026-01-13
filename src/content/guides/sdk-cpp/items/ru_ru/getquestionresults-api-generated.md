## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Нет |  |
| userId | string | Нет |  |
| startDate | string | Нет |  |
| questionId | string | Нет |  |
| questionIds | string | Нет |  |
| skip | double | Нет |  |

## Ответ

Возвращает: [`GetQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResults_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример getQuestionResults'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("/articles/2025/new-feature"));
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("alice@example.com"));
boost::optional<utility::string_t> startDate = boost::optional<utility::string_t>(U("2025-01-01T00:00:00Z"));
boost::optional<utility::string_t> questionId = boost::optional<utility::string_t>(U("q-789"));
boost::optional<utility::string_t> questionIds = boost::optional<utility::string_t>(U("q-789,q-790"));
boost::optional<double> skip = boost::optional<double>(10.0);

api->getQuestionResults(tenantId, urlId, userId, startDate, questionId, questionIds, skip)
.then([](pplx::task<std::shared_ptr<GetQuestionResults_200_response>> t){
    try {
        std::shared_ptr<GetQuestionResults_200_response> resp = t.get();
        if (resp) {
            auto copy = std::make_shared<GetQuestionResults_200_response>(*resp);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

---