## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| batchJobId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportStatusResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getApiExportStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> batchId = utility::string_t(U("export-batch-2026-06-19"));
boost::optional<utility::string_t> sso = utility::string_t(U("audit@my-tenant-123.com"));
api->getApiExportStatus(batchId, sso)
    .then([](std::shared_ptr<ModerationExportStatusResponse> resp) {
        if (!resp) return;
        auto statusCopy = std::make_shared<ModerationExportStatusResponse>(*resp);
    });
[inline-code-end]

---