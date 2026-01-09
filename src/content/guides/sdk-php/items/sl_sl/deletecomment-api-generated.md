## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| contextUserId | string | query | Ne |  |
| isLive | boolean | query | Ne |  |

## Odgovor

Vrne: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteComment200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer deleteComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtorizacijo API ključa: api_key
// Odkomentirajte spodnjo vrstico za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lasten HTTP odjemalec, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$context_user_id = 'context_user_id_example'; // string
$is_live = True; // bool

try {
    $result = $apiInstance->deleteComment($tenant_id, $id, $context_user_id, $is_live);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---