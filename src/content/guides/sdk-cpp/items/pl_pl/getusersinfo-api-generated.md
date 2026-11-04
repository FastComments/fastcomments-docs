Informacje zbiorcze o użytkownikach dla tenanta. Na podstawie userIds zwraca informacje wyświetlane z User / SSOUser.
Używane przez widżet komentarzy do wzbogacenia użytkowników, którzy właśnie pojawili się poprzez zdarzenie obecności.
Bez kontekstu strony: prywatność jest egzekwowana jednolicie (prywatne profile są maskowane).

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| ids | string | Tak |  |

## Odpowiedź

Zwraca: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // przetwórz odpowiedź
    }catch(const std::exception&){
        // obsłuż błąd
    }
});
[inline-code-end]