## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | forespørgsel | Ja |  |
| id | string | sti | Ja |  |

## Respons

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Eksempel

[inline-code-attrs-start title = 'updateEmailTemplate Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at sætte et præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, videregiv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_email_template_body = new \FastComments\Client\Model\UpdateEmailTemplateBody(); // \FastComments\Client\Model\UpdateEmailTemplateBody

try {
    $result = $apiInstance->updateEmailTemplate($tenant_id, $id, $update_email_template_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]