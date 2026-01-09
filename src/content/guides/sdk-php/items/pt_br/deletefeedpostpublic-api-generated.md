## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| postId | string | path | Sim |  |
| broadcastId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteFeedPostPublic200Response.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteFeedPostPublic($tenant_id, $post_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]