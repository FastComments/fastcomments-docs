req
tenantId
afterId

## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| afterId | string | query | Não |  |
| limit | integer | query | Não |  |
| tags | array | query | Não |  |
| sso | string | query | Não |  |
| isCrawler | boolean | query | Não |  |
| includeUserInfo | boolean | query | Não |  |

## Resposta

Retorna: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsPublic200Response.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getFeedPostsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado por padrão.
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