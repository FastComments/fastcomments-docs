## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetBannedUsersFromCommentResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getBanUsersFromComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará como predeterminado.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // cadena
$sso = 'sso_example'; // cadena

try {
    $result = $apiInstance->getBanUsersFromComment($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getBanUsersFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]