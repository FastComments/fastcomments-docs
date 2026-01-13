## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα replaceTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
utility::string_t packageId = utility::string_t("pkg-456");
ReplaceTenantPackageBody body;
body.name = utility::string_t("Premium Plan");
body.planId = utility::string_t("premium_monthly");
body.seats = 25;
body.billingEmail = utility::string_t("billing@acme.com");
boost::optional<utility::string_t> notes = boost::optional<utility::string_t>(utility::string_t("Migrated subscription"));
body.notes = notes;
api->replaceTenantPackage(tenantId, packageId, body).then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task){
    try{
        auto resp = task.get();
        if(resp){
            auto resultCopy = std::make_shared<FlagCommentPublic_200_response>(*resp);
        }
    } catch(const std::exception&){
    }
});
[inline-code-end]

---