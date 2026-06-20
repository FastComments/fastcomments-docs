## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantUserResponse.php)

## Primjer

[inline-code-attrs-start title = 'Primjer createTenantUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte autorizaciju API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Otkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao zadani.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_tenant_user_body = new \FastComments\Client\Model\CreateTenantUserBody(); // \FastComments\Client\Model\CreateTenantUserBody

try {
    $result = $apiInstance->createTenantUser($tenant_id, $create_tenant_user_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]