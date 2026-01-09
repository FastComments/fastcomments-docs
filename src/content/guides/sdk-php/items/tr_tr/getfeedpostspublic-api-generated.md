req
tenantId
afterId

## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| afterId | string | query | Hayır |  |
| limit | integer | query | Hayır |  |
| tags | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeUserInfo | boolean | query | Hayır |  |

## Yanıt

Dönüş: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsPublic200Response.php)

## Örnek

[inline-code-attrs-start title = 'getFeedPostsPublic Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` arayüzünü uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır; varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
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