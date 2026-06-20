## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Ja |  |

## Respons

Retourneert: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionConfigResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'createQuestionConfig Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateQuestionConfigBody body;
body.questionText = U("Do you want to receive our newsletter?");
body.isRequired = true;
body.moderatorEmail = boost::optional<utility::string_t>(U("moderator@example.com"));
api->createQuestionConfig(tenantId, body)
.then([](std::shared_ptr<CreateQuestionConfigResponse> resp){
    auto result = std::make_shared<CreateQuestionConfigResponse>(*resp);
    return result;
})
.then([](std::shared_ptr<CreateQuestionConfigResponse> finalResp){
    (void)finalResp;
})
.wait();
[inline-code-end]

---