## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| locale | string | query | No |  |

## Risposta

Restituisce: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/RenderEmailTemplateResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di renderEmailTemplate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autenticazione con API key: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Se necessario, decommentare la riga sottostante per impostare il prefisso (es. Bearer) per l'API key
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se vuoi usare un client HTTP personalizzato, passa il client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale: `GuzzleHttp\Client` verrà usato come predefinito.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // stringa
$render_email_template_body = new \FastComments\Client\Model\RenderEmailTemplateBody(); // \FastComments\Client\Model\RenderEmailTemplateBody
$locale = 'locale_example'; // stringa

try {
    $result = $apiInstance->renderEmailTemplate($tenant_id, $render_email_template_body, $locale);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->renderEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]