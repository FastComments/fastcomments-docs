Komentujący z przeszłości na stronie, którzy NIE są obecnie online. Sortowanie według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję „Members”.
Paginacja kursorowa po commenterName: serwer przechodzi po częściowym indeksie {tenantId, urlId, commenterName}
od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak | Identyfikator URL strony (czyszczony po stronie serwera). |
| afterName | string | query | Nie | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | Nie | Kursor rozstrzygający remisy: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagany, gdy ustawiono afterName, aby remisy nazw nie powodowały utraty wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta http, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To opcjonalne, `GuzzleHttp\Client` będzie używany domyślnie.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identyfikator URL strony (czyszczony po stronie serwera).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi.
    'after_user_id' => 'after_user_id_example', // string | Kursor rozstrzygający remisy: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagany, gdy ustawiono afterName, aby remisy nazw nie powodowały utraty wpisów.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]