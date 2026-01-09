req
tenantId
afterId

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| afterId | string | query | Не |  |
| limit | integer | query | Не |  |
| tags | array | query | Не |  |
| sso | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeUserInfo | boolean | query | Не |  |

## Одговор

Враћа: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsPublic200Response.php)

## Пример

[inline-code-attrs-start title = 'getFeedPostsPublic Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$after_id = 'after_id_example'; // string
$limit = 56; // int
$tags = array('tags_example'); // string[]
$sso = 'sso_example'; // string
$is_crawler = True; // bool
$include_user_info = True; // bool

try {
    $result = $apiInstance->getFeedPostsPublic($tenant_id, $after_id, $limit, $tags, $sso, $is_crawler, $include_user_info);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]