## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| approved | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetCommentApprovedResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postSetCommentApprovalStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$approved = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postSetCommentApprovalStatus($comment_id, $approved, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentApprovalStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]