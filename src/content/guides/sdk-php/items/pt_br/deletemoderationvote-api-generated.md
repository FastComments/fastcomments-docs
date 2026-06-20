---
## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| voteId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteModerationVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$vote_id = 'vote_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteModerationVote($comment_id, $vote_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->deleteModerationVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---