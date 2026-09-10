### API FastComments

FastComments udostępnia API do interakcji z wieloma zasobami. Twórz integracje z naszą platformą lub nawet własne klienty!

W tej dokumentacji znajdziesz wszystkie obsługiwane zasoby API, udokumentowane wraz z typami żądań i odpowiedzi.

Dla klientów Enterprise, cały dostęp do API jest rejestrowany w dzienniku audytu.

### Wygenerowane SDK

FastComments teraz generuje [Specyfikację API](https://fastcomments.com/js/swagger.json) z naszego kodu (nie jest jeszcze kompletna, ale zawiera wiele API).

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

API jest uwierzytelniane poprzez przekazanie Twojego [klucza API](https://fastcomments.com/auth/my-account/api-secret) jako nagłówka `X-API-KEY` lub parametru zapytania `API_KEY`. Będziesz także potrzebował swojego `tenantId` do wykonywania wywołań API. Można go pobrać z tej samej strony co klucz API.

### Uwaga dotycząca bezpieczeństwa

Te trasy są przeznaczone do wywoływania z **serwera**. __NIE__ wywołuj ich z przeglądarki. Zrobienie tego ujawni Twój klucz API – da to pełny dostęp do Twojego konta każdemu, kto może zobaczyć kod źródłowy strony!

#### Opcja uwierzytelniania pierwsza – Nagłówki

- Nagłówek: `X-API-KEY`
- Nagłówek: `X-TENANT-ID`

#### Opcja uwierzytelniania druga – Parametry zapytania

- Parametr zapytania: `API_KEY`
- Parametr zapytania: `tenantId`

#### Opcja uwierzytelniania trzecia – Token OAuth Bearer

- Nagłówek: `Authorization: Bearer fcat_...`

Aplikacje łączące się przez [serwer MCP](https://docs.fastcomments.com/guide-llm-kit.html) uzyskują token poprzez OAuth zamiast klucza API. Ten token działa na każdym endpointzie tutaj. Tenant jest domyślnie określony przez token, więc `tenantId` jest opcjonalny, ale musi pasować do tokenu, jeśli jest podany. Żądania `GET` wymagają zakresu `read`, a wszystkie inne metody wymagają zakresu `write`. Odkrywanie zaczyna się pod adresem `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Odczytywanie własnych zapisów

FastComments zapewnia dostępność Active-Active. Żądania z Twojego centrum danych są kierowane do [najbliższego punktu obecności](https://sophon.fastcomments.com/) względem Ciebie. Działa to automatycznie i zazwyczaj możesz obserwować semantykę „read‑your‑write”. Jeśli chcesz mieć pewność, że odczytasz własne zapisy, możesz przypiąć swoje żądania do określonego regionu, używając tego regionu jako hosta API (choć zazwyczaj nie jest to potrzebne w większości integracji):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Zauważ, że jeśli to zrobisz, możesz chcieć zdefiniować mechanizm awaryjny, ponieważ w przeszłości wycofywaliśmy węzły wejściowe i używamy nowych nazw przy przełączaniu.

---