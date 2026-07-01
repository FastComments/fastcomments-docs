---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## Antwort

Rückgabe: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSubscriptionAPIResponse.h)

## Beispiel

[inline-code-attrs-start title = 'deleteSubscription Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->deleteSubscription(utility::string_t(U("my-tenant-123")), utility::string_t(U("sub-456")), boost::optional<utility::string_t>(utility::string_t(U("user@example.com"))))
    .then([](std::shared_ptr<DeleteSubscriptionAPIResponse> resp){
        if (resp) {
        }
    });
[inline-code-end]
---