## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AwardUserBadgeResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример putAwardBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$badge_id = 'badge_id_example'; // строка
$user_id = 'user_id_example'; // строка
$comment_id = 'comment_id_example'; // строка
$broadcast_id = 'broadcast_id_example'; // строка
$sso = 'sso_example'; // строка

try {
    $result = $apiInstance->putAwardBadge($badge_id, $user_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putAwardBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]