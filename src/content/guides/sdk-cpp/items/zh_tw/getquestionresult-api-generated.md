## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetQuestionResult_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResult_200_response.h)

## 範例

[inline-code-attrs-start title = 'getQuestionResult 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t questionId = U("question-456");
boost::optional<utility::string_t> requestor = U("user@example.com");

auto task = api->getQuestionResult(tenantId, questionId)
    .then([requestor](pplx::task<std::shared_ptr<GetQuestionResult_200_response>> prev) -> std::shared_ptr<GetQuestionResult_200_response> {
        try {
            auto resp = prev.get();
            if (!resp) return std::make_shared<GetQuestionResult_200_response>();
            (void)requestor;
            return resp;
        } catch (const std::exception &) {
            return std::make_shared<GetQuestionResult_200_response>();
        }
    });
[inline-code-end]

---