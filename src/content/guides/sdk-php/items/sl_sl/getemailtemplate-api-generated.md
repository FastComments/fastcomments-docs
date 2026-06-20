## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vrača: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEmailTemplateResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getEmailTemplate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfiguriraj overjanje z API ključem: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Po potrebi odkomentirajte spodnje, da nastavite predpono (npr. Bearer) za API ključ
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lastnega HTTP odjemalca, podajte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To ni obvezno, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getEmailTemplate($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---