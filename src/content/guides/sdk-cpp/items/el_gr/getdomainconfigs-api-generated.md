## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigsResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getDomainConfigs'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> region = boost::none;
utility::string_t tenantId = U("my-tenant-123");
api->getDomainConfigs(tenantId)
.then([tenantId, region](std::shared_ptr<GetDomainConfigsResponse> resp) {
    auto result = resp ? resp : std::make_shared<GetDomainConfigsResponse>();
    std::cout << "Received domain configs for " << tenantId << std::endl;
    return result;
})
.then([](std::shared_ptr<GetDomainConfigsResponse> finalResp) {
    if (finalResp) {
        std::cout << "Configs available" << std::endl;
    } else {
        std::cout << "No configs returned" << std::endl;
    }
})
.wait();
[inline-code-end]

---