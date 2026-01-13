## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vrača: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Primer

[inline-code-attrs-start title = 'updateTenantPackage Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtorizacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lastnega http odjemalca, posredujte odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je opcijsko, kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_tenant_package_body = new \FastComments\Client\Model\UpdateTenantPackageBody(); // \FastComments\Client\Model\UpdateTenantPackageBody

try {
    $result = $apiInstance->updateTenantPackage($tenant_id, $id, $update_tenant_package_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]