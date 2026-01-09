## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createAPISSOUserData | CreateAPISSOUserData | Ja |  |

## Respons

Returnerer: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddSSOUserAPIResponse.h)

## Eksempel

[inline-code-attrs-start title = 'addSSOUser Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto createData = std::make_shared<CreateAPISSOUserData>();
createData->email = utility::string_t(U("alice@example.com"));
createData->externalUserId = utility::string_t(U("okta|987654321"));
createData->displayName = boost::optional<utility::string_t>(U("Alice Johnson"));
createData->roles = std::vector<utility::string_t>{ U("moderator"), U("editor") };
api->addSSOUser(tenantId, createData)
.then([](pplx::task<std::shared_ptr<AddSSOUserAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            (void)resp;
        }
    } catch (...) {
    }
});
[inline-code-end]

---