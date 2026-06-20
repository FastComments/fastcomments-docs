Lista stron dla najemcy. Używane przez klienta desktopowego FChat do wypełnienia jego listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozstrzygniętej konfiguracji niestandardowej dla każdej strony.
Strony, które wymagają SSO, są filtrowane pod kątem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | int32_t | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | bool | No |  |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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