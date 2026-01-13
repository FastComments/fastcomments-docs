## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| errorId | string | path | Da |  |

## Odgovor

Vrača: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer deleteEmailTemplateRenderError'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Nastavite avtorizacijo z API ključem: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je opcijsko, kot privzeti bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$error_id = 'error_id_example'; // string

try {
    $result = $apiInstance->deleteEmailTemplateRenderError($tenant_id, $id, $error_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteEmailTemplateRenderError: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]