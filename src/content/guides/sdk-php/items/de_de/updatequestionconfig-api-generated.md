## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Beispiel

[inline-code-attrs-start title = 'updateQuestionConfig Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Authentifizierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Falls benötigt, die folgende Zeile einkommentieren, um ein Präfix (z. B. Bearer) für den API-Schlüssel festzulegen
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_question_config_body = new \FastComments\Client\Model\UpdateQuestionConfigBody(); // \FastComments\Client\Model\UpdateQuestionConfigBody

try {
    $result = $apiInstance->updateQuestionConfig($tenant_id, $id, $update_question_config_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateQuestionConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]