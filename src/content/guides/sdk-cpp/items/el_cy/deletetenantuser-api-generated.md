## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| options | const DeleteTenantUserOptions& | Yes |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteTenantUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DeleteTenantUserOptions options;
options.reason = boost::optional<utility::string_t>(U("User requested deletion"));

api->deleteTenantUser(U("my-tenant-123"), U("user@example.com"), options)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
        (void)resp;
    });
[inline-code-end]