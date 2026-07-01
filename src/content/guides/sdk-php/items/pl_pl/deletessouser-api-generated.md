## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |
| deleteComments | boolean | query | Nie |  |
| commentDeleteMode | string | query | Nie |  |

## Odpowiedź

Zwraca: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSSOUserAPIResponse.php)

## Przykład

[inline-code-attrs-start title = 'deleteSSOUser Przykład'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Konfiguracja autoryzacji klucza API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzeba
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Jeśli chcesz użyć własnego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // To jest opcjonalne, `GuzzleHttp\Client` zostanie użyty domyślnie.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$options = [
    'delete_comments' => True, // bool
    'comment_delete_mode' => 'comment_delete_mode_example', // string
];


try {
    $result = $apiInstance->deleteSSOUser($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]