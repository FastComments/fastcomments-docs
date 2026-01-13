## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니오 |  |
| questionIds | vector<string | 아니오 |  |
| urlId | string | 아니오 |  |
| startDate | datetime | 아니오 |  |
| forceRecalculate | bool | 아니오 |  |
| minValue | double | 아니오 |  |
| maxValue | double | 아니오 |  |
| limit | double | 아니오 |  |

## 응답

반환: [`CombineCommentsWithQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CombineCommentsWithQuestionResults_200_response.h)

## 예제

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> questionId = boost::optional<utility::string_t>(U("q-98765"));
boost::optional<std::vector<utility::string_t>> questionIds = boost::optional<std::vector<utility::string_t>>(std::vector<utility::string_t>{U("q-98765"), U("q-12345")});
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("/posts/2024/interesting-article"));
boost::optional<utility::datetime> startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2024-01-01T00:00:00Z")));
boost::optional<bool> forceRecalculate = boost::optional<bool>(true);
boost::optional<double> minValue = boost::optional<double>(0.0);
boost::optional<double> maxValue = boost::optional<double>(1.0);
boost::optional<double> limit = boost::optional<double>(100.0);

api->combineCommentsWithQuestionResults(tenantId, questionId, questionIds, urlId, startDate, forceRecalculate, minValue, maxValue, limit)
.then([](pplx::task<std::shared_ptr<CombineCommentsWithQuestionResults_200_response>> task){
    try {
        auto response = task.get();
        auto output = response ? response : std::make_shared<CombineCommentsWithQuestionResults_200_response>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---