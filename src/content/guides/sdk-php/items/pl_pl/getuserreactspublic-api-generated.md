## Parametry

| Name | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| postIds | array | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UserReactsResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserReactsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta implementującego `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, `GuzzleHttp\Client` będzie używany jako domyślny.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_ids = array('post_ids_example'); // string[]
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserReactsPublic($tenant_id, $post_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserReactsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]