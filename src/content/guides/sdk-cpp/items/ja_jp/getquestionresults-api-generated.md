## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | No |  |
| userId | string | No |  |
| startDate | string | No |  |
| questionId | string | No |  |
| questionIds | string | No |  |
| skip | double | No |  |

## レスポンス

戻り値: [`GetQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResults_200_response.h)

## 例

[inline-code-attrs-start title = 'getQuestionResults の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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