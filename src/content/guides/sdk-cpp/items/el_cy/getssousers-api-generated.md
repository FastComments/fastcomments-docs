## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| skip | int32_t | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUsersResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSSOUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<int32_t> skip = 25;
api->getSSOUsers(tenantId, skip)
    .then([](pplx::task<std::shared_ptr<GetSSOUsersResponse>> task) {
        try {
            auto resp = task.get();
            if (!resp) resp = std::make_shared<GetSSOUsersResponse>();
            (void)resp;
        } catch (const std::exception& ex) {
            (void)ex;
        }
    });
[inline-code-end]

---