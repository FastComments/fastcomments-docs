## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## Απάντηση

Επιστρέφει: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα putDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("comments.myapp.com");
auto params = std::make_shared<UpdateDomainConfigParams>();
params->displayName = utility::string_t(U("My App Comments"));
params->enabled = boost::optional<bool>(true);
params->contactEmail = boost::optional<utility::string_t>(U("admin@myapp.com"));
api->putDomainConfig(tenantId, domainToUpdate, *params)
.then([](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto updated = resp;
        }
    } catch (const std::exception& e) {
    }
});
[inline-code-end]

---