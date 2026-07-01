## 參數

| 名稱 | 類型 | 必須 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetQuestionResultsOptions& | Yes |  |

## 回應

返回: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultsResponse.h)

## 範例

[inline-code-attrs-start title = 'getQuestionResults 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetQuestionResultsOptions options;
options.questionId = boost::optional<utility::string_t>(U("question-456"));
options.includeDeleted = boost::optional<bool>(false);
api->getQuestionResults(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetQuestionResultsResponse>> t) {
        auto response = t.get();
        // 處理回應
    });
[inline-code-end]