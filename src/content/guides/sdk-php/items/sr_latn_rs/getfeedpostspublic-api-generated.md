zahtev
tenantId
afterId

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| afterId | string | query | Ne |  |
| limit | integer | query | Ne |  |
| tags | array | query | Ne |  |
| sso | string | query | Ne |  |
| isCrawler | boolean | query | Ne |  |
| includeUserInfo | boolean | query | Ne |  |

## Odgovor

Vraća: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsPublic200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer getFeedPostsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, koristiće se `GuzzleHttp\Client` kao podrazumevani.
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