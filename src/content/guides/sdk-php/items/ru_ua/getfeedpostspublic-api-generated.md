req
tenantId
afterId

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| afterId | string | query | Нет |  |
| limit | integer | query | Нет |  |
| tags | array | query | Нет |  |
| sso | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |
| includeUserInfo | boolean | query | Нет |  |

## Ответ

Возвращает: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PublicFeedPostsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getFeedPostsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
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