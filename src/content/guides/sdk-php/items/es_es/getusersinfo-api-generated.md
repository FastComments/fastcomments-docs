Información de usuario en lote para un tenant. Dados userIds, devuelve la información de visualización de User / SSOUser.
Usado por el widget de comentarios para enriquecer a los usuarios que acaban de aparecer mediante un evento de presencia.
Sin contexto de página: la privacidad se aplica de manera uniforme (los perfiles privados se enmascaran).

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| ids | string | query | Sí | userIds delimitados por comas. |

## Respuesta

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si quieres usar un cliente http personalizado, pasa tu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará por defecto.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds delimitados por comas.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]