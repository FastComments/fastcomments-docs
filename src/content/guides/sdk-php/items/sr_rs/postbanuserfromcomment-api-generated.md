## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| banEmail | boolean | query | Не |  |
| banEmailDomain | boolean | query | Не |  |
| banIP | boolean | query | Не |  |
| deleteAllUsersComments | boolean | query | Не |  |
| bannedUntil | string | query | Не |  |
| isShadowBan | boolean | query | Не |  |
| updateId | string | query | Не |  |
| banReason | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевани.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // стринг
$comment_id = 'comment_id_example'; // стринг
$options = [
    'ban_email' => True, // бул
    'ban_email_domain' => True, // бул
    'ban_ip' => True, // бул
    'delete_all_users_comments' => True, // бул
    'banned_until' => 'banned_until_example', // стринг
    'is_shadow_ban' => True, // бул
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

---