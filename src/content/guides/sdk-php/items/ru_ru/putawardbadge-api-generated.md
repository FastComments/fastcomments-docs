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

Возвращает: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AwardUserBadgeResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример putAwardBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать пользовательский HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, `GuzzleHttp\Client` будет использован по умолчанию.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$badge_id = 'badge_id_example'; // string
$options = [
    'user_id' => 'user_id_example', // string
    'comment_id' => 'comment_id_example', // string
    'broadcast_id' => 'broadcast_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->putAwardBadge($tenant_id, $badge_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putAwardBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---