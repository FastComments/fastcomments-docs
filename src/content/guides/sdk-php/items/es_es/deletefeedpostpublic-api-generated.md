## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| postId | string | path | Sí |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteFeedPostPublic200Response.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará como valor predeterminado.
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

---