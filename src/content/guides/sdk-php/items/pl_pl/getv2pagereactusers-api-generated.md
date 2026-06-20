## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| id | string | query | Tak |  |

## Odpowiedź

Zwraca: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV2PageReactUsersResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getV2PageReactUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta implementującego `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getV2PageReactUsers($tenant_id, $url_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV2PageReactUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]