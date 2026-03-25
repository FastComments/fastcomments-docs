---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Da |  |
| userId | string | Ne |  |

## Odgovor

Vraća: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateSubscriptionAPIResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer updateSubscription'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t subscriptionId = U("sub-456");
UpdateAPIUserSubscriptionData updateData;
auto fallbackResp = std::make_shared<UpdateSubscriptionAPIResponse>();
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
api->updateSubscription(tenantId, subscriptionId, updateData, userId)
    .then([](pplx::task<std::shared_ptr<UpdateSubscriptionAPIResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) {
                // obradi resp
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---