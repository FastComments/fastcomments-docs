### API FastComments

FastComments udostępnia API do interakcji z wieloma zasobami. Twórz integracje z naszą platformą lub nawet własnych klientów!

W tej dokumentacji znajdziesz wszystkie zasoby obsługiwane przez API wraz z udokumentowanymi typami żądań i odpowiedzi.

Dla klientów Enterprise cały dostęp do API jest rejestrowany w Audit Log.

### Wygenerowane SDK

FastComments generuje teraz [API Spec](https://fastcomments.com/js/swagger.json) z naszego kodu (nie jest jeszcze kompletna, ale obejmuje wiele interfejsów API).

Mamy również SDKi dla popularnych języków:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Uwierzytelnianie

API uwierzytelnia się poprzez przekazanie Twojego [api key](https://fastcomments.com/auth/my-account/api-secret) jako nagłówka `X-API-KEY` lub parametru zapytania `API_KEY`. Do wykonywania wywołań API będziesz także potrzebować `tenantId`. Można go pobrać z tej samej strony co Twój api key.

### Uwaga dotycząca bezpieczeństwa

Te trasy są przeznaczone do wywoływania z **serwera**. __NIE__ wywołuj ich z przeglądarki. Spowoduje to ujawnienie Twojego `API key` — zapewni to pełny dostęp do Twojego konta każdemu, kto może zobaczyć kod źródłowy strony!

#### Opcja uwierzytelniania jedna - Nagłówki

- Nagłówek: `X-API-KEY`
- Nagłówek: `X-TENANT-ID`

#### Opcja uwierzytelniania druga - Parametry zapytania

- Parametr zapytania: `API_KEY`
- Parametr zapytania: `tenantId`