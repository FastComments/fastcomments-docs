Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde sin liste over rum.
Kræver, at `enableFChat` er sat til true i den opløste brugerdefinerede konfiguration for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| cursor | string | Nej |  |
| limit | int32_t | Nej |  |
| q | string | Nej |  |
| sortBy | PagesSortBy | Nej |  |
| hasComments | bool | Nej |  |

## Svar

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> cursor = utility::string_t(U("cursor_abc"));
boost::optional<int32_t> limit = 50;
boost::optional<utility::string_t> q = utility::string_t(U("status:published"));
boost::optional<PagesSortBy> sortBy = PagesSortBy::NEWEST;
boost::optional<bool> hasComments = true;
api->getPagesPublic(tenantId, cursor, limit, q, sortBy, hasComments)
.then([](std::shared_ptr<GetPublicPagesResponse> resp){
    if (!resp) resp = std::make_shared<GetPublicPagesResponse>();
})
.wait();
[inline-code-end]

---