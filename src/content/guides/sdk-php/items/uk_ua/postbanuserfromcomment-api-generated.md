## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| banEmail | boolean | query | Ні |  |
| banEmailDomain | boolean | query | Ні |  |
| banIP | boolean | query | Ні |  |
| deleteAllUsersComments | boolean | query | Ні |  |
| bannedUntil | string | query | Ні |  |
| isShadowBan | boolean | query | Ні |  |
| updateId | string | query | Ні |  |
| banReason | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Приклад

[inline-code-attrs-start title = 'postBanUserFromComment Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, `GuzzleHttp\Client` буде використаний за замовчуванням.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$comment_id = 'comment_id_example'; // рядок
$options = [
    'ban_email' => True, // булевий
    'ban_email_domain' => True, // булевий
    'ban_ip' => True, // булевий
    'delete_all_users_comments' => True, // булевий
    'banned_until' => 'banned_until_example', // рядок
    'is_shadow_ban' => True, // булевий
    'update_id' => 'update_id_example', // рядок
    'ban_reason' => 'ban_reason_example', // рядок
    'sso' => 'sso_example', // рядок
];


try {
    $result = $apiInstance->postBanUserFromComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]