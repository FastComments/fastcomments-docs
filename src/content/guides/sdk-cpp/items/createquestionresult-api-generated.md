## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## Response

Returns: [`CreateQuestionResult_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionResult_200_response.h)

## Example

[inline-code-attrs-start title = 'createQuestionResult Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateQuestionResultBody body;
body.questionId = U("q-789");
body.userEmail = U("alice@example.com");
body.answer = U("I believe this is correct");
body.score = boost::optional<int>(92);
body.metadata = std::make_shared<std::map<utility::string_t, utility::string_t>>();
(*body.metadata)[U("platform")] = U("web");

api->createQuestionResult(tenantId, body)
.then([](pplx::task<std::shared_ptr<CreateQuestionResult_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) { (void)resp; }
    } catch (const std::exception&) { }
});
[inline-code-end]
