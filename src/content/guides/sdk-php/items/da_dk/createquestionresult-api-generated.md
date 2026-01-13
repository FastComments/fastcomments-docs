## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateQuestionResult200Response.php)

## Eksempel

[inline-code-attrs-start title = 'createQuestionResult Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentar nedenfor for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, giv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_question_result_body = new \FastComments\Client\Model\CreateQuestionResultBody(); // \FastComments\Client\Model\CreateQuestionResultBody

try {
    $result = $apiInstance->createQuestionResult($tenant_id, $create_question_result_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createQuestionResult: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]