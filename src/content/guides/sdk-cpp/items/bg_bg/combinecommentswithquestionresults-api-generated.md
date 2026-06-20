---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| questionId | string | Не |  |
| questionIds | vector<string | Не |  |
| urlId | string | Не |  |
| startDate | datetime | Не |  |
| forceRecalculate | bool | Не |  |
| minValue | double | Не |  |
| maxValue | double | Не |  |
| limit | double | Не |  |

## Отговор

Връща: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CombineQuestionResultsWithCommentsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример combineCommentsWithQuestionResults'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> questionId = utility::conversions::to_string_t("q-456");
std::vector<utility::string_t> qlist = { utility::conversions::to_string_t("q-101"), utility::conversions::to_string_t("q-102") };
boost::optional<std::vector<utility::string_t>> questionIds = qlist;
boost::optional<utility::string_t> urlId = utility::conversions::to_string_t("page-789");
boost::optional<utility::datetime> startDate = utility::datetime::from_string(utility::conversions::to_string_t("2025-01-01T00:00:00Z"));
boost::optional<bool> forceRecalculate = true;
boost::optional<double> minValue = 0.0;
boost::optional<double> maxValue = 5.0;
boost::optional<double> limit = 100.0;

api->combineCommentsWithQuestionResults(
    tenantId,
    questionId,
    questionIds,
    urlId,
    startDate,
    forceRecalculate,
    minValue,
    maxValue,
    limit
).then([](std::shared_ptr<CombineQuestionResultsWithCommentsResponse> resp){
    auto result = resp ? resp : std::make_shared<CombineQuestionResultsWithCommentsResponse>();
    return result;
});
[inline-code-end]

---