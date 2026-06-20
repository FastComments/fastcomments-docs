---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## Απόκριση

Επιστρέφει: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchDomainConfigResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("example.com");
PatchDomainConfigParams patchParams;
patchParams.enableComments = boost::optional<bool>(true);
patchParams.moderatorEmail = boost::optional<utility::string_t>(U("moderator@example.com"));
auto task = api->patchDomainConfig(tenantId, domainToUpdate, patchParams)
    .then([](std::shared_ptr<PatchDomainConfigResponse> resp) {
        auto result = resp ? resp : std::make_shared<PatchDomainConfigResponse>();
        return result;
    });
task.wait();
[inline-code-end]

---