### API FastComments

FastComments udostępnia API do interakcji z wieloma zasobami. Buduj integracje z naszą platformą, a nawet twórz własnych klientów!

W tej dokumentacji znajdziesz wszystkie zasoby obsługiwane przez API udokumentowane wraz z ich typami żądań i odpowiedzi.

Dla klientów Enterprise wszystkie dostępy do API są rejestrowane w Dzienniku audytu.

### Wygenerowane SDK

FastComments teraz generuje [Specyfikacja API](https://fastcomments.com/js/swagger.json) z naszego kodu (to nie jest jeszcze w pełni kompletne, ale obejmuje wiele API).

Mamy również SDK dla popularnych języków:

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

API jest uwierzytelniane poprzez przekazanie Twojego [api key](https://fastcomments.com/auth/my-account/api-secret) jako nagłówka `X-API-KEY` lub parametru zapytania `API_KEY`. Do wywołań API będziesz także potrzebować `tenantId`. Można go pobrać z tej samej strony co Twój klucz API.

### Uwaga dotycząca bezpieczeństwa

Te trasy mają być wywoływane z poziomu **serwera**. __NIE__ wywołuj ich z przeglądarki. Zrobienie tego ujawni Twój klucz API — da to pełny dostęp do Twojego konta każdemu, kto może zobaczyć kod źródłowy strony!

#### Opcja uwierzytelniania pierwsza - nagłówki

- Nagłówek: `X-API-KEY`
- Nagłówek: `X-TENANT-ID`

#### Opcja uwierzytelniania druga - parametry zapytania

- Parametr zapytania: `API_KEY`
- Parametr zapytania: `tenantId`

### Odczytywanie własnych zapisów

FastComments zapewnia dostępność Active-Active. Żądania z Twojego centrum danych są kierowane do [najbliższego punktu obecności](https://sophon.fastcomments.com/) względem Twojego. To dzieje się automatycznie i zazwyczaj możesz zaobserwować semantykę read-your-write. Jeśli chcesz mieć pewność, że odczytasz własne zapisy, możesz przypiąć swoje żądania do określonego regionu, używając tego regionu jako hosta API (jednak zazwyczaj nie jest to potrzebne dla większości integracji):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Zwróć uwagę, że jeśli to zrobisz, możesz chcieć zdefiniować fallback, ponieważ w przeszłości wycofywaliśmy węzły wejściowe i używaliśmy nowych nazw podczas przełączeń.