## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|-----------|-------------|
| tenantId | string | Sì |  |
| options | const GetPendingWebhookEventCountOptions& | Sì |  |

## Risposta

Restituisce: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEventCountResponse.h)

## Esempio

[inline-code-attrs-start title = 'getPendingWebhookEventCount Esempio'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
GetPendingWebhookEventCountOptions opts;
opts.filter = boost::optional<utility::string_t>(utility::conversions::to_string_t("active"));
api->getPendingWebhookEventCount(tenantId, opts)
    .then([](std::shared_ptr<GetPendingWebhookEventCountResponse> resp){
        auto count = resp->getCount();
    });
[inline-code-end]