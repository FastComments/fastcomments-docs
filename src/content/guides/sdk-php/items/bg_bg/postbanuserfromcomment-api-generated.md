## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Отговор

Returns: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Пример

[inline-code-attrs-start title = 'Пример за postBanUserFromComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\ModerationApi(
//     // Ако искате да използвате персонализиран HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
//     // Това е опционално, `GuzzleHttp\Client` ще бъде използвано по подразбиране.
//     new GuzzleHttp\Client()
// );

$tenant_id = 'tenant_id_example'; // стринг
$comment_id = 'comment_id_example'; // стринг
$options = [
    'ban_email' => True, // булев
    'ban_email_domain' => True, // булев
    'ban_ip' => True, // булев
    'delete_all_users_comments' => True, // булев
    'banned_until' => 'banned_until_example', // стринг
    'is_shadow_ban' => True, // булев
    'update_id' => 'update_id_example', // стринг
    'ban_reason' => 'ban_reason_example', // стринг
    'sso' => 'sso_example', // стринг
];


try {
    $result = $apiInstance->postBanUserFromComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]