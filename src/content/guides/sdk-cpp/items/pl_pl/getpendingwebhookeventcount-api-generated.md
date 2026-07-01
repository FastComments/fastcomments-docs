---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| options | const GetPendingWebhookEventCountOptions& | Tak |  |

## Odpowiedź

Zwraca: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEventCountResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getPendingWebhookEventCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
GetPendingWebhookEventCountOptions opts;
opts.filter = boost::optional<utility::string_t>(utility::conversions::to_string_t("active"));
api->getPendingWebhookEventCount(tenantId, opts)
    .then([](std::shared_ptr<GetPendingWebhookEventCountResponse> resp){
        auto count = resp->getCount();
    });
[inline-code-end]

---