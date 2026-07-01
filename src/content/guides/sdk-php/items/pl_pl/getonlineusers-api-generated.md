Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (czyszczony po stronie serwera). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Tiebreaker kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy ustawiono afterName, aby nie pomijało wpisów przy remisie nazw. |

## Odpowiedź

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Przykład

[inline-code-attrs-start title = 'getOnlineUsers Przykład'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć własnego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, `GuzzleHttp\Client` będzie użyty jako domyślny.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | identyfikator URL strony (czyszczony po stronie serwera).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi.
    'after_user_id' => 'after_user_id_example', // string | Tiebreaker kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy ustawiono afterName, aby nie pomijało wpisów przy remisie nazw.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---