### Błędy 401 Nieautoryzowane

Jeśli otrzymujesz błędy 401 podczas korzystania z uwierzytelnionego API:

1. **Sprawdź swój klucz API**: Upewnij się, że używasz prawidłowego klucza API z panelu FastComments
2. **Zweryfikuj identyfikator najemcy**: Upewnij się, że identyfikator najemcy odpowiada Twojemu kontu
3. **Format klucza API**: Klucz API powinien być ustawiony jako nagłówek `x-api-key` w wspólnej konfiguracji:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Używanie niewłaściwego API**: Upewnij się, że używasz `DefaultAPI` (nie `PublicAPI`) dla uwierzytelnionych wywołań

### Problemy z tokenami SSO

Jeśli tokeny SSO nie działają:

1. **Używaj trybu zabezpieczonego w produkcji**: Zawsze używaj `FastCommentsSSO.createSecure()` z kluczem API w środowisku produkcyjnym
2. **Tylko po stronie serwera**: Generuj bezpieczne tokeny SSO na serwerze, nigdy nie ujawniaj klucza API klientom
3. **Sprawdź dane użytkownika**: Upewnij się, że wszystkie wymagane pola (id, email, username) są dostarczone
4. **Wygaśnięcie tokenu**: Bezpieczne tokeny SSO zawierają znacznik czasu i mogą wygasać. Generuj nowe tokeny w razie potrzeby.

### Błędy SSL/TLS

Jeśli napotkasz błędy SSL/TLS:

1. Upewnij się, że plik Info.plist Twojej aplikacji zezwala na połączenia HTTPS do fastcomments.com
2. Sprawdź, czy nie używasz wyjątków App Transport Security, które mogą blokować połączenie