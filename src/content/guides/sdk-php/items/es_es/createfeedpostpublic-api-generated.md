## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|----------|-----------|-------------|
| tenantId | string | path | Sí |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional; `GuzzleHttp\Client` se empleará como predeterminado.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$create_feed_post_params = new \FastComments\Client\Model\CreateFeedPostParams(); // \FastComments\Client\Model\CreateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->createFeedPostPublic($tenant_id, $create_feed_post_params, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]