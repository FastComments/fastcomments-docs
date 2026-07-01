## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Yes |  |
| options | const GetPendingWebhookEventCountOptions& | Yes |  |

## Réponse

Renvoie : [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEventCountResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getPendingWebhookEventCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
GetPendingWebhookEventCountOptions opts;
opts.filter = boost::optional<utility::string_t>(utility::conversions::to_string_t("active"));
api->getPendingWebhookEventCount(tenantId, opts)
    .then([](std::shared_ptr<GetPendingWebhookEventCountResponse> resp){
        auto count = resp->getCount();
    });
[inline-code-end]