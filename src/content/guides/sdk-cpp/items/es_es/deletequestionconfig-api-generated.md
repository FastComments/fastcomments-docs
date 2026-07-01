## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteQuestionConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto configId = utility::conversions::to_string_t("question-config-456");

api->deleteQuestionConfig(tenantId, configId)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // manejar eliminación exitosa
    })
    .then([](pplx::task<void> t) {
        try {
            t.get();
        } catch (const std::exception&) {
            // manejar error
        }
    });
[inline-code-end]