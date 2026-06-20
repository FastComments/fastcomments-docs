---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| updateComments | string | query | Ne |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer uporabe updateTenantUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfiguriraj avtorizacijo z API ključem: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Če je potrebno, odkomentirajte spodnjo vrstico, da nastavite predpono (npr. Bearer) za API ključ
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lastnega HTTP klienta, posredujte klienta, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_tenant_user_body = new \FastComments\Client\Model\UpdateTenantUserBody(); // \FastComments\Client\Model\UpdateTenantUserBody
$update_comments = 'update_comments_example'; // string

try {
    $result = $apiInstance->updateTenantUser($tenant_id, $id, $update_tenant_user_body, $update_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---