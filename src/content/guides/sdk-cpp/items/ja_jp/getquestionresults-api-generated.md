---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | いいえ |  |
| userId | string | いいえ |  |
| startDate | string | いいえ |  |
| questionId | string | いいえ |  |
| questionIds | string | いいえ |  |
| skip | double | いいえ |  |

## レスポンス

戻り値: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultsResponse.h)

## 例

[inline-code-attrs-start title = 'getQuestionResults の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> urlId(U("page-456"));
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> startDate(U("2023-01-01T00:00:00Z"));
boost::optional<utility::string_t> questionId(U("q-789"));
boost::optional<utility::string_t> questionIds(U("q-789,q-790"));
boost::optional<double> skip(10.0);
api->getQuestionResults(tenantId, urlId, userId, startDate, questionId, questionIds, skip)
.then([](pplx::task<std::shared_ptr<GetQuestionResultsResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetQuestionResultsResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetQuestionResultsResponse>();
    }
});
[inline-code-end]

---