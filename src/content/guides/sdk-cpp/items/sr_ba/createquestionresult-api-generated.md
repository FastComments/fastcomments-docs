## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createQuestionResultBody | CreateQuestionResultBody | Da |  |

## Odgovor

Vraća: [`CreateQuestionResult_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionResult_200_response.h)

## Primjer

[inline-code-attrs-start title = 'Primjer createQuestionResult'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateQuestionResultBody createQuestionResultBody;
createQuestionResultBody.questionId = U("question-789");
createQuestionResultBody.userEmail = U("student@example.com");
createQuestionResultBody.selectedOption = U("B");
createQuestionResultBody.isCorrect = true;
createQuestionResultBody.score = 9.5;
createQuestionResultBody.metadata = boost::optional<utility::string_t>(U("{\"session\":\"quiz-2026-01\"}"));
auto task = api->createQuestionResult(tenantId, createQuestionResultBody)
.then([](pplx::task<std::shared_ptr<CreateQuestionResult_200_response>> t){
    try {
        auto result = t.get();
        auto copy = std::make_shared<CreateQuestionResult_200_response>(*result);
        (void)copy;
    } catch (...) {}
});
task.wait();
[inline-code-end]

---