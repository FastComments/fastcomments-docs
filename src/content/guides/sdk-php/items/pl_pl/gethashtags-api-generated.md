## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| page | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetHashTagsResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getHashTags'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj autoryzację klucza API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta implementującego `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne; domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$page = 3.4; // float

try {
    $result = $apiInstance->getHashTags($tenant_id, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getHashTags: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]