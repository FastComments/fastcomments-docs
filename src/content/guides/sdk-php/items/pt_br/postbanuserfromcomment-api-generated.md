## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| commentId | string | path | Sim |  |
| banEmail | boolean | query | Não |  |
| banEmailDomain | boolean | query | Não |  |
| banIP | boolean | query | Não |  |
| deleteAllUsersComments | boolean | query | Não |  |
| bannedUntil | string | query | Não |  |
| isShadowBan | boolean | query | Não |  |
| updateId | string | query | Não |  |
| banReason | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Exemplo

[inline-code-attrs-start title = 'postBanUserFromComment Exemplo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\ModerationApi(
//     // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
//     // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
//     new GuzzleHttp\Client()
// );

$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$options = [
    'ban_email' => True, // bool
    'ban_email_domain' => True, // bool
    'ban_ip' => True, // bool
    'delete_all_users_comments' => True, // bool
    'banned_until' => 'banned_until_example', // string
    'is_shadow_ban' => True, // bool
    'update_id' => 'update_id_example', // string
    'ban_reason' => 'ban_reason_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->postBanUserFromComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]