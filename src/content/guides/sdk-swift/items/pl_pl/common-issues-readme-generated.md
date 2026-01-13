### 401 — Brak autoryzacji

Jeśli otrzymujesz błędy 401 podczas korzystania z uwierzytelnionego API:

1. **Sprawdź swój klucz API**: Upewnij się, że używasz poprawnego klucza API z panelu FastComments
2. **Zweryfikuj tenant ID**: Upewnij się, że tenant ID odpowiada Twojemu kontu
3. **Format klucza API**: Klucz API powinien być ustawiony w kliencie API:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Używanie złego API**: Upewnij się, że używasz `DefaultAPI` (nie `PublicAPI`) dla wywołań uwierzytelnionych

### Problemy z tokenami SSO

Jeśli tokeny SSO nie działają:

1. **Używaj trybu bezpiecznego w produkcji**: Zawsze używaj `FastCommentsSSO.createSecure()` z kluczem API w środowisku produkcyjnym
2. **Tylko po stronie serwera**: Generuj bezpieczne tokeny SSO na swoim serwerze, nigdy nie ujawniaj klucza API klientom
3. **Sprawdź dane użytkownika**: Upewnij się, że wszystkie wymagane pola (id, email, username) są dostarczone
4. **Wygasanie tokenów**: Bezpieczne tokeny SSO zawierają znacznik czasu i mogą wygasnąć. Generuj nowe tokeny w razie potrzeby.

### Błędy SSL/TLS

Jeśli napotkasz błędy SSL/TLS:

1. Upewnij się, że Info.plist Twojej aplikacji pozwala na połączenia HTTPS z fastcomments.com
2. Sprawdź, czy nie używasz wyjątków App Transport Security, które mogą blokować połączenie