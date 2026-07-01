## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createQuestionResultBody | CreateQuestionResultBody | Sí |  |

## Respuesta

Devuelve: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionResultResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createQuestionResult'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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