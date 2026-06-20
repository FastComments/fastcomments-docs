## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
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

[inline-code-attrs-start title = 'Exemplo de postBanUserFromComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se você quiser usar um cliente HTTP personalizado, passe um cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional; `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$ban_email = True; // bool
$ban_email_domain = True; // bool
$ban_ip = True; // bool
$delete_all_users_comments = True; // bool
$banned_until = 'banned_until_example'; // string
$is_shadow_ban = True; // bool
$update_id = 'update_id_example'; // string
$ban_reason = 'ban_reason_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postBanUserFromComment($comment_id, $ban_email, $ban_email_domain, $ban_ip, $delete_all_users_comments, $banned_until, $is_shadow_ban, $update_id, $ban_reason, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]