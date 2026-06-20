## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Так |  |
| userId | string | query | Ні |  |
| commentId | string | query | Ні |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AwardUserBadgeResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад putAwardBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$badge_id = 'badge_id_example'; // рядок
$user_id = 'user_id_example'; // рядок
$comment_id = 'comment_id_example'; // рядок
$broadcast_id = 'broadcast_id_example'; // рядок
$sso = 'sso_example'; // рядок

try {
    $result = $apiInstance->putAwardBadge($badge_id, $user_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putAwardBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]