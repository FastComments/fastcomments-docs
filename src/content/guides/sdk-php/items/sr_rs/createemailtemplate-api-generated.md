## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Vraća: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateEmailTemplateResponse.php)

## Primer

[inline-code-attrs-start title = 'createEmailTemplate Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurišite autorizaciju API ključa: api_key
// Okomentarišite dole da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ako želite koristiti prilagođeni http klijent, prosledite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će se koristiti kao podrazumevano.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$create_email_template_body = new \FastComments\Client\Model\CreateEmailTemplateBody(); // \FastComments\Client\Model\CreateEmailTemplateBody


try {
    $result = $apiInstance->createEmailTemplate($tenant_id, $create_email_template_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]