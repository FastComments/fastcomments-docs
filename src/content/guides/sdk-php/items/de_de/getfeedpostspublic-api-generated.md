req
tenantId
afterId

## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| afterId | string | query | Nein |  |
| limit | integer | query | Nein |  |
| tags | array | query | Nein |  |
| sso | string | query | Nein |  |
| isCrawler | boolean | query | Nein |  |
| includeUserInfo | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsPublic200Response.php)

## Beispiel

[inline-code-attrs-start title = 'getFeedPostsPublic Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
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