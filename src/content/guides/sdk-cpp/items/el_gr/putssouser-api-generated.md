## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Ναι |  |
| updateComments | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα putSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
UpdateAPISSOUserData updateData;
updateData.displayName = U("Jane Doe");
updateData.email = U("user@example.com");
boost::optional<bool> updateComments = true;
api->putSSOUser(tenantId, id, updateData, updateComments)
.then([](std::shared_ptr<PutSSOUserAPIResponse> resp){
    if(!resp){ std::cout << "putSSOUser returned null\n"; return; }
    auto copy = std::make_shared<PutSSOUserAPIResponse>(*resp);
    std::cout << "SSO user updated successfully\n";
});
[inline-code-end]

---