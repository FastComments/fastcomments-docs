## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| options | const GetPendingWebhookEventCountOptions& | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEventCountResponse.h)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEventCount Παράδειγμα'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
GetPendingWebhookEventCountOptions opts;
opts.filter = boost::optional<utility::string_t>(utility::conversions::to_string_t("active"));
api->getPendingWebhookEventCount(tenantId, opts)
    .then([](std::shared_ptr<GetPendingWebhookEventCountResponse> resp){
        auto count = resp->getCount();
    });
[inline-code-end]