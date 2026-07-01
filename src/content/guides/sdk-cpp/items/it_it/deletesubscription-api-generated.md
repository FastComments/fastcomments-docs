## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| userId | string | No |  |

## Risposta

Restituisce: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSubscriptionAPIResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteSubscription'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->deleteSubscription(utility::string_t(U("my-tenant-123")), utility::string_t(U("sub-456")), boost::optional<utility::string_t>(utility::string_t(U("user@example.com"))))
    .then([](std::shared_ptr<DeleteSubscriptionAPIResponse> resp){
        if (resp) {
        }
    });
[inline-code-end]