---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| updateTenantBody | UpdateTenantBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateTenant'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto id = utility::string_t(U("tenant-0001"));
auto bodyPtr = std::make_shared<UpdateTenantBody>();
bodyPtr->setDisplayName(utility::string_t(U("Acme Corporation")));
bodyPtr->setAdminEmail(utility::string_t(U("admin@acme.com")));
bodyPtr->setIsActive(true);
bodyPtr->setMaxUsers(boost::optional<int>(250));
api->updateTenant(tenantId, id, *bodyPtr)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto msg = resp->getMessage();
            std::wcout << msg << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
    }
});
[inline-code-end]

---