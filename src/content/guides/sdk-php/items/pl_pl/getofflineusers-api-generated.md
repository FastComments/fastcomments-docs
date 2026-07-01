Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|--------------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (czyszczony po stronie serwera). |
| afterName | string | query | No | Kursor: przekazać nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Rozstrzygacz kursora: przekazać nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby powiązania nazw nie pomijały wpisów. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Przykład'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć własnego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, `GuzzleHttp\Client` zostanie użyty jako domyślny.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identyfikator URL strony (czyszczony po stronie serwera).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: przekazać nextAfterName z poprzedniej odpowiedzi.
    'after_user_id' => 'after_user_id_example', // string | Rozstrzygacz kursora: przekazać nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby powiązania nazw nie pomijały wpisów.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]