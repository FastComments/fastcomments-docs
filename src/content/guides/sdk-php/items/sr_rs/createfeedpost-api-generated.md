## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| broadcastId | string | query | Не |  |
| isLive | boolean | query | Не |  |
| doSpamCheck | boolean | query | Не |  |
| skipDupCheck | boolean | query | Не |  |

## Одговор

Враћа: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPost200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример createFeedPost'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Ако је потребно, откоментаришите доле како бисте подесили префикс (нпр. Bearer) за API кључ
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Ово је опционално, подразумевано ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_feed_post_params = new \FastComments\Client\Model\CreateFeedPostParams(); // \FastComments\Client\Model\CreateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // string
$is_live = True; // bool
$do_spam_check = True; // bool
$skip_dup_check = True; // bool

try {
    $result = $apiInstance->createFeedPost($tenant_id, $create_feed_post_params, $broadcast_id, $is_live, $do_spam_check, $skip_dup_check);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createFeedPost: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]