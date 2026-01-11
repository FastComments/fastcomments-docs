## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| doSpamCheck | boolean | query | No |  |
| isLive | boolean | query | No |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Przykład

[inline-code-attrs-start title = 'Przykład updateComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj uwierzytelnianie kluczem API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie użyty zostanie `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$body = new \FastComments\Client\Model\PickAPICommentUpdatableCommentFields(); // \FastComments\Client\Model\PickAPICommentUpdatableCommentFields
$context_user_id = 'context_user_id_example'; // string
$do_spam_check = True; // bool
$is_live = True; // bool

try {
    $result = $apiInstance->updateComment($tenant_id, $id, $body, $context_user_id, $do_spam_check, $is_live);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]