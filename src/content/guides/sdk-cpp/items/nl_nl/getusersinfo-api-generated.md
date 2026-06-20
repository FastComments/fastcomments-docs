Bulk gebruikersinformatie voor een tenant. Gegeven userIds, retourneer weergavegegevens van User / SSOUser.
Wordt gebruikt door de reactie-widget om gebruikers te verrijken die zojuist verschenen via een presence-event.
Geen pagina-context: privacy wordt uniform gehandhaafd (privéprofielen worden afgeschermd).

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Respons

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getUsersInfo Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> statusFilter = U("active");
api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try {
        auto res = t.get();
        if (res) {
            auto responseCopy = std::make_shared<PageUsersInfoResponse>(*res);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

---