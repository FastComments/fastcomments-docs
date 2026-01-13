## Παράμετροι

| Όνομα | Τύπος | Θέση | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Απάντηση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateQuestionConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Αποσχολιάστε παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για API key, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
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