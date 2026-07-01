## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| urlIdWS | string | query | Tak |  |
| userIds | string | query | Tak |  |

## Odpowiedź

Zwraca: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserPresenceStatusesResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserPresenceStatuses'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć własnego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To opcjonalne, `GuzzleHttp\Client` będzie użyty jako domyślny.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // ciąg
$url_id_ws = 'url_id_ws_example'; // ciąg
$user_ids = 'user_ids_example'; // ciąg


try {
    $result = $apiInstance->getUserPresenceStatuses($tenant_id, $url_id_ws, $user_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserPresenceStatuses: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]