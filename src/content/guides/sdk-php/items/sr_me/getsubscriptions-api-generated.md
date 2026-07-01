## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |

## Odgovor

Vraća: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSubscriptionsAPIResponse.php)

## Primjer

[inline-code-attrs-start title = 'Primjer getSubscriptions'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Konfigurišite autorizaciju API ključa: api_key
// Odkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ako želite koristiti prilagođeni http klijent, proslijedite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će se koristiti kao podrazumevani.
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

---