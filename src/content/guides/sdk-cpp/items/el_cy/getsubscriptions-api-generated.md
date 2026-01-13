## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| userId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSubscriptionsAPIResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSubscriptions'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> userId = utility::conversions::to_string_t("user@example.com");
api->getSubscriptions(tenantId, userId)
.then([](std::shared_ptr<GetSubscriptionsAPIResponse> resp){
    if(!resp) return;
    auto copy = std::make_shared<GetSubscriptionsAPIResponse>(*resp);
})
.wait();
[inline-code-end]

---