## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| postIds | array | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsStats200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer getFeedPostsStats'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti po meri HTTP odjemalec, posredujte odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je izbirno; kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_ids = array('post_ids_example'); // string[]
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getFeedPostsStats($tenant_id, $post_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsStats: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---