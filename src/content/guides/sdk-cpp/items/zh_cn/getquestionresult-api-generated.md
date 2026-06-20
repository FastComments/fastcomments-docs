## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultResponse.h)

## 示例

[inline-code-attrs-start title = 'getQuestionResult 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t questionId = U("question-789");
boost::optional<utility::string_t> requestedBy = boost::optional<utility::string_t>(U("user@example.com"));

api->getQuestionResult(tenantId, questionId)
.then([requestedBy](pplx::task<std::shared_ptr<GetQuestionResultResponse>> task) -> std::shared_ptr<GetQuestionResultResponse> {
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<GetQuestionResultResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetQuestionResultResponse>();
    }
});
[inline-code-end]

---