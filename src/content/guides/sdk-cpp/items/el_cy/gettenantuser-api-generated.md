## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUserResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
api->getTenantUser(tenantId, userId)
    .then([](pplx::task<std::shared_ptr<GetTenantUserResponse>> task) {
        try {
            auto response = task.get();
            // Χρησιμοποιήστε την απόκριση όπως χρειάζεται
        } catch (const std::exception&) {
            // Διαχείριση σφάλματος
        }
    });
[inline-code-end]