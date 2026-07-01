## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'updateQuestionResult 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto questionId = utility::string_t(U("question-456"));
UpdateQuestionResultBody body;
body.result = U("approved");
body.note = boost::optional<utility::string_t>(U("Reviewed by admin"));
api->updateQuestionResult(tenantId, questionId, body)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
        try {
            auto respPtr = std::make_shared<APIEmptyResponse>(*t.get());
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---