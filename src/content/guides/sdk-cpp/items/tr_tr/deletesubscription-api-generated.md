## Parameters

| Ad | Tür | Gereklidir | Açıklama |
|------|------|------------|-----------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| userId | string | Hayır |  |

## Response

Döndürür: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSubscriptionAPIResponse.h)

## Example

[inline-code-attrs-start title = 'deleteSubscription Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->deleteSubscription(utility::string_t(U("my-tenant-123")), utility::string_t(U("sub-456")), boost::optional<utility::string_t>(utility::string_t(U("user@example.com"))))
    .then([](std::shared_ptr<DeleteSubscriptionAPIResponse> resp){
        if (resp) {
        }
    });
[inline-code-end]