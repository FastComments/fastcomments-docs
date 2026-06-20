## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| reviewed | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de postSetCommentReviewStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional; se utilizará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$reviewed = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postSetCommentReviewStatus($comment_id, $reviewed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentReviewStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]