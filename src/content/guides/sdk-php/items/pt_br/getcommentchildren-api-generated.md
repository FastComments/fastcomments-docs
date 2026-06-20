## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIChildCommentsResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentChildren'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentChildren($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCommentChildren: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---