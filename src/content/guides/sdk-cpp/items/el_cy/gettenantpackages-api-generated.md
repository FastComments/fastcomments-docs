## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| skip | double | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTenantPackages_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackages_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantPackages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
auto task = api->getTenantPackages(tenantId, skip).then([](pplx::task<std::shared_ptr<GetTenantPackages_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            (void)resp;
        }
    } catch (const std::exception&) {
        auto fallback = std::make_shared<GetTenantPackages_200_response>();
        (void)fallback;
    }
});
[inline-code-end]

---