---
## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AdjustVotesResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'postAdjustCommentVotes Ejemplo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$adjust_comment_votes_params = new \FastComments\Client\Model\AdjustCommentVotesParams(); // \FastComments\Client\Model\AdjustCommentVotesParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postAdjustCommentVotes($comment_id, $adjust_comment_votes_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postAdjustCommentVotes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---