## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateTenantUserBody | UpdateTenantUserBody | Oui |  |
| updateComments | string | Non |  |

## Réponse

Renvoie : [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple updateTenantUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("jane.doe@example.com");
auto bodyPtr = std::make_shared<UpdateTenantUserBody>();
bodyPtr->setEmail(U("jane.doe@example.com"));
bodyPtr->setDisplayName(U("Jane Doe"));
boost::optional<utility::string_t> updateComments = U("Normalized display name");

api->updateTenantUser(tenantId, userId, *bodyPtr, updateComments)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Tenant user updated successfully\n";
        else std::cout << "Update returned no data\n";
    } catch (const std::exception& ex) {
        std::cerr << "Update failed: " << ex.what() << '\n';
    }
});
[inline-code-end]

---