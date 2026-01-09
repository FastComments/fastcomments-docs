req
tenantId
afterId

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| afterId | string | query | Nie |  |
| limit | integer | query | Nie |  |
| tags | array | query | Nie |  |
| sso | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |
| includeUserInfo | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsPublic200Response.php)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getFeedPostsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie używany będzie `GuzzleHttp\Client`.
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