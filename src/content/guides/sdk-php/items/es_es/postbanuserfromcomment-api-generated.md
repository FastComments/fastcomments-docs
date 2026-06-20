## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de postBanUserFromComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // cadena
$ban_email = True; // booleano
$ban_email_domain = True; // booleano
$ban_ip = True; // booleano
$delete_all_users_comments = True; // booleano
$banned_until = 'banned_until_example'; // cadena
$is_shadow_ban = True; // booleano
$update_id = 'update_id_example'; // cadena
$ban_reason = 'ban_reason_example'; // cadena
$sso = 'sso_example'; // cadena

try {
    $result = $apiInstance->postBanUserFromComment($comment_id, $ban_email, $ban_email_domain, $ban_ip, $delete_all_users_comments, $banned_until, $is_shadow_ban, $update_id, $ban_reason, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]