---
Sammel-Benutzerinformationen für einen Mandanten. Anhand gegebener userIds werden Anzeigeinformationen aus User / SSOUser zurückgegeben.
Wird vom Kommentar-Widget verwendet, um Benutzer anzureichern, die gerade durch ein Präsenzereignis erschienen sind.
Kein Seitenkontext: Der Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Antwort

Gibt zurück: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getUsersInfo Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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