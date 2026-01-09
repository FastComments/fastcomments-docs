FastComments wykorzystuje zmienne środowiskowe do konfiguracji. Poniższa lista przedstawia wszystkie obsługiwane zmienne istotne dla wdrożeń On-Prem.

| Zmienna                        | Domyślna                   | Opis                                                                                                                                                | Wymagane | Przykłady lub wartości                                |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Typ środowiska.                                                                                                                                     | Yes      | production, dev                                       |
| MONGO_URI                      |                             | URI połączenia z bazą danych.                                                                                                                      | Yes      |                                                       |
| MONGO_ENABLE_SSL               | false                       | Aktywuje użycie SSL do połączenia z bazą danych.                                                                                                  | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Włącza walidację certyfikatu względem CA podczas łączenia z Mongo.                                                                                 | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | Plik PEM z CA dla SSL Mongo.                                                                                                                       | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Adres e-mail, na który powinny trafiać ważne powiadomienia systemowe.                                                                              | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Sól do haszowania adresów IP.                                                                                                                      | Yes      |                                                       |
| SESSION_SECRET                 |                             | Klucz używany do podpisywania sesji.                                                                                                               | Yes      |                                                       |
| SESSION_STORE_SECRET           |                             | Klucz używany do podpisywania/haszowania sesji w przechowywaniu. Musi różnić się od SESSION_SECRET.                                                | Yes      |                                                       |
| HOSTNAME                       |                             | Nazwa hosta, na którym jest wdrożony FastComments (panel administracyjny itp.). NIE powinna zawierać numeru portu ani protokołu.                    | Yes      | example.com                                           |
| HOST_ADDR                      |                             | Dostępny URI, pod którym wdrożony jest FastComments (panel administracyjny itp.).                                                                  | Yes      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Ścieżka w lokalnym systemie plików do pliku konfiguracyjnego e-mail (SMTP, mapowania domen/dostawców itp.).                                         | Yes      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | Robot FastComments          | Nagłówek "From Name" w e-mailu.                                                                                                                     | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo w stopce e-maila.                                                                                                                              | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Zastąpienie wartości "defaultTransport" w EMAIL_CONFIG_PATH. Przydatne przy wdrażaniu tego samego pliku konfiguracyjnego w różnych środowiskach.     | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ID konta na fastcomments.com. Używane do zarejestrowania klucza licencyjnego.                                                                      | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Klucz licencyjny on-prem.                                                                                                                           | No       |                                                       |
| GIPHY_API_KEY                  |                             | Klucz API Giphy. Jeśli nie jest określony, powinieneś utworzyć regułę konfiguracyjną wyłączającą wybór gifów.                                       | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Używane w integracji z Giphy. Może być także nadpisane regułami dostosowania widgetu.                                                              | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Używane do funkcji opartych na OpenAI, np. opcjonalnego wykrywania spamu opartego na GPT.                                                           | No       |                                                       |
| CDN_HOST_ADDR                  |                             | Nazwa hosta, z którego będą pobierane zasoby. Domyślnie wartość HOSTNAME.                                                                          | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Nazwa hosta, z którego pobierane są duże pliki (np. eksporty). Domyślnie wartość CDN_HOST_ADDR.                                                     | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Gdzie powinny być przechowywane duże pliki, takie jak eksporty.                                                                                    | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Nazwa hosta, z którego powinny być wysyłane e-maile.                                                                                                | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Nazwa ciasteczka fastcomments.                                                                                                                      | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Wartość pola "hostname" w ciasteczku. Zalecane poprzedzenie kropką.                                                                                | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Używane do przesyłania plików użytkowników, avatarów itp. Domyślnie używany lokalny system plików, jeśli niezdefiniowane.                           | No       |                                                       |
| S3_SECRET_KEY                  |                             | Używane do przesyłania plików użytkowników, avatarów itp.                                                                                          | No       |                                                       |
| S3_REGION                      |                             | Używane do przesyłania plików użytkowników, avatarów itp.                                                                                          | No       |                                                       |
| S3_BUCKET                      |                             | Używane do przesyłania plików użytkowników, avatarów itp.                                                                                          | No       |                                                       |
| S3_HOST                        |                             | Używane do przesyłania plików użytkowników, avatarów itp.                                                                                          | No       |                                                       |
| CACHE_DIR                      |                             | Lokalizacja do przechowywania opcjonalnego cache offline, na wypadek gdy DB jest niedostępna. Okresowo odświeżane z 100 najbardziej popularnymi wątkami komentarzy. | No       |                                                       |
| BACKUP_DIR                     |                             | Lokalizacja do przechowywania danych na wypadek niedostępności DB. Jeśli komentarz zostanie przesłany, gdy DB jest niedostępna, trafia tutaj i jest przetwarzany później. | No       |                                                       |

Zauważ, że wszystkie zmienne związane z domenami używają końcówek `_HOST` lub `_ADDR`. Różnica to:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Ścieżka w `EMAIL_CONFIG_PATH` powinna wskazywać na plik JSON o następującym przykładowym formacie:

[inline-code-attrs-start title = 'Konfiguracja e-mail'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "defaultDKIM": {
        "domainName": "mycompany.org",
        "keySelector": "2024",
        "privateKey": "-----BEGIN PRIVATE KEY-----\nABCDEFG\n-----END PRIVATE KEY-----"
    },
    "providerTransports": {
        "yahoo.com": "specialTransport"
    },
    "defaultTransport": "mailgun",
    "transports": {
        "mailgun": {
            "host": "smtp.mailgun.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@somewhere.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        },
        "specialTransport": {
            "host": "smtp.someplace.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@example.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        }
    }
}
[inline-code-end]

W powyższym przykładzie definiujemy domyślny transport e-mail `SMTP` o nazwie `mailgun`. Definiujemy także specjalny transport, którego używamy konkretnie dla e-maili `@yahoo.com`. W niektórych scenariuszach pożądane jest użycie określonego dostawcy lub adresu IP nadawcy dla danej domeny w celu dostrojenia dostarczalności. To jest opcjonalne.

### DocumentDB

Podczas łączenia z `DocumentDB` warto ustawić `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem`, aby być zgodnym z domyślnymi ustawieniami.