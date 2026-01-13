## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nein |  |
| questionIds | array | query | Nein |  |
| urlId | string | query | Nein |  |
| timeBucket | string | query | Nein |  |
| startDate | string | query | Nein |  |
| forceRecalculate | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateQuestionResults200Response.php)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für aggregateQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Authentifizierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie das Kommentarzeichen unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, falls erforderlich
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = array('question_ids_example'); // string[]
$url_id = 'url_id_example'; // string
$time_bucket = new \FastComments\Client\Model\\FastComments\Client\Model\AggregateTimeBucket(); // \FastComments\Client\Model\AggregateTimeBucket
$start_date = new \DateTime('2013-10-20T19:20:30+01:00'); // \DateTime
$force_recalculate = True; // bool

try {
    $result = $apiInstance->aggregateQuestionResults($tenant_id, $question_id, $question_ids, $url_id, $time_bucket, $start_date, $force_recalculate);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregateQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]