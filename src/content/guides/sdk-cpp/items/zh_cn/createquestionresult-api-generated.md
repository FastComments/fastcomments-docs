## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## 响应

返回：[`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionResultResponse.h)

## 示例

[inline-code-attrs-start title = 'createQuestionResult 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateQuestionResultBody body;
body.questionId = U("question-456");
body.result = U("approved");
body.comment = boost::optional<utility::string_t>(U("Looks good"));
api->createQuestionResult(tenantId, body)
    .then([=](pplx::task<std::shared_ptr<CreateQuestionResultResponse>> t) {
        try {
            auto resp = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---