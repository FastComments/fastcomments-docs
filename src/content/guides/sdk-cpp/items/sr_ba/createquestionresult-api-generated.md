## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createQuestionResultBody | CreateQuestionResultBody | Da |  |

## Odgovor

Vraća: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionResultResponse.h)

## Primjer

[inline-code-attrs-start title = 'createQuestionResult Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateQuestionResultBody body;
body.setQuestionId(U("q-9876"));
body.setUserId(U("user-42"));
body.setAnswer(U("I prefer option B"));
body.setScore(4);
body.setUserEmail(boost::optional<utility::string_t>(U("jane.doe@example.com")));
body.setNotes(boost::optional<utility::string_t>(U("Followed up via email")));
api->createQuestionResult(tenantId, body)
.then([](pplx::task<std::shared_ptr<CreateQuestionResultResponse>> t){
    try {
        auto resp = t.get();
        if (!resp) return;
        auto resultCopy = std::make_shared<CreateQuestionResultResponse>(*resp);
        (void)resultCopy;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---