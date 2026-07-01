## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| badgeId | string | query | Да |  |
| userId | string | query | Нет |  |
| commentId | string | query | Нет |  |
| broadcastId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возврат: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/RemoveUserBadgeResponse.php)

## Пример

[inline-code-attrs-start title = 'putRemoveBadge пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать пользовательский HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, `GuzzleHttp\Client` будет использоваться по умолчанию.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // строка
$badge_id = 'badge_id_example'; // строка
$options = [
    'user_id' => 'user_id_example', // строка
    'comment_id' => 'comment_id_example', // строка
    'broadcast_id' => 'broadcast_id_example', // строка
    'sso' => 'sso_example', // строка
];


try {
    $result = $apiInstance->putRemoveBadge($tenant_id, $badge_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putRemoveBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]