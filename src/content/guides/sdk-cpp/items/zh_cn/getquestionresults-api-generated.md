---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetQuestionResultsOptions& | 是 |  |

## 响应

返回：[`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultsResponse.h)

## 示例

[inline-code-attrs-start title = 'getQuestionResults 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetQuestionResultsOptions options;
options.questionId = boost::optional<utility::string_t>(U("question-456"));
options.includeDeleted = boost::optional<bool>(false);
api->getQuestionResults(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetQuestionResultsResponse>> t) {
        auto response = t.get();
        // 处理响应
    });
[inline-code-end]

---