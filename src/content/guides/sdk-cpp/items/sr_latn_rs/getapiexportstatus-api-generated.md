## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| batchJobId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportStatusResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getApiExportStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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