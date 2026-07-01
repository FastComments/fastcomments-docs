## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | double | No |  |

## Απόκριση

Επιστρέφει: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackagesResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantPackages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> skip = 20.0;
api->getTenantPackages(tenantId, skip)
    .then([](std::shared_ptr<GetTenantPackagesResponse> resp) {
        (void)resp;
    });
[inline-code-end]

---