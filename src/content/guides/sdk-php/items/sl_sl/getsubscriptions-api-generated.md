## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |

## Odgovor

Vrne: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSubscriptionsAPIResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getSubscriptions'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfiguriraj avtentikacijo z API ključem: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Če je potrebno, odkomentirajte spodnjo vrstico za nastavitev predpone (npr. Bearer) za API ključ
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To ni obvezno, privzeti bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->getSubscriptions($tenant_id, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSubscriptions: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]