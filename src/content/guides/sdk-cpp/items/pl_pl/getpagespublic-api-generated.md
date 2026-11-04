Wyświetla listę stron dla tenanta. Używane przez klienta desktopowego FChat do wypełnienia listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązanej niestandardowej konfiguracji dla każdej strony.
Strony wymagające SSO są filtrowane względem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| options | const GetPagesPublicOptions& | Tak |  |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // process response if needed
    }catch(const std::exception&){
        // handle error if needed
    }
});
[inline-code-end]