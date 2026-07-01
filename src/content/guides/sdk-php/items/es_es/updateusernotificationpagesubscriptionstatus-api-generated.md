Enable o deshabilita notificaciones para una página. Cuando los usuarios están suscritos a una página, se crean notificaciones para nuevos comentarios raíz, y también

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| urlId | string | query | Sí |  |
| url | string | query | Sí |  |
| pageTitle | string | query | Sí |  |
| subscribedOrUnsubscribed | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationPageSubscriptionStatusResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo updateUserNotificationPageSubscriptionStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si deseas usar un cliente http personalizado, pasa tu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // cadena
$url_id = 'url_id_example'; // cadena
$url = 'url_example'; // cadena
$page_title = 'page_title_example'; // cadena
$subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example'; // cadena
$sso = 'sso_example'; // cadena


try {
    $result = $apiInstance->updateUserNotificationPageSubscriptionStatus($tenant_id, $url_id, $url, $page_title, $subscribed_or_unsubscribed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]