Informacje zbiorcze o użytkownikach dla tenanta. Na podstawie userIds zwraca informacje wyświetlane z User / SSOUser.
Używane przez widżet komentarzy do wzbogacenia użytkowników, którzy właśnie pojawili się przez zdarzenie obecności.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (prywatne profile są maskowane).

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| ids | string | query | Tak | Identyfikatory userIds rozdzielone przecinkami. |

## Odpowiedź

Zwraca: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta http, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Identyfikatory userIds rozdzielone przecinkami.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]