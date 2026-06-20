## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример postBanUserUndo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно — по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$ban_user_undo_params = new \FastComments\Client\Model\BanUserUndoParams(); // \FastComments\Client\Model\BanUserUndoParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postBanUserUndo($ban_user_undo_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserUndo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]