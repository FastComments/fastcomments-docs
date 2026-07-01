## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| includeByUserIdAndEmail | boolean | query | Нет |  |
| includeByIP | boolean | query | Нет |  |
| includeByEmailDomain | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`PreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PreBanSummary.php)

## Пример

[inline-code-attrs-start title = 'getPreBanSummary Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать собственный HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // строка
$comment_id = 'comment_id_example'; // строка
$options = [
    'include_by_user_id_and_email' => True, // булево
    'include_by_ip' => True, // булево
    'include_by_email_domain' => True, // булево
    'sno' => 'sso_example', // строка
];


try {
    $result = $apiInstance->getPreBanSummary($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]