## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| contextUserId | string | query | Nee |  |
| doSpamCheck | boolean | query | Nee |  |
| isLive | boolean | query | Nee |  |

## Response

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'updateComment Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-sleutel autorisatie configureren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Haal hieronder de commentaar weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` zal standaard worden gebruikt.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$updatable_comment_params = new \FastComments\Client\Model\UpdatableCommentParams(); // \FastComments\Client\Model\UpdatableCommentParams
$context_user_id = 'context_user_id_example'; // string
$do_spam_check = True; // bool
$is_live = True; // bool

try {
    $result = $apiInstance->updateComment($tenant_id, $id, $updatable_comment_params, $context_user_id, $do_spam_check, $is_live);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]