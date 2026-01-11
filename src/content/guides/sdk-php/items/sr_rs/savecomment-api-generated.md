## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| isLive | boolean | query | Не |  |
| doSpamCheck | boolean | query | Не |  |
| sendEmails | boolean | query | Не |  |
| populateNotifications | boolean | query | Не |  |

## Одговор

Враћа: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SaveComment200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример saveComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуришите овлашћење API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Откоментирајте доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_comment_params = new \FastComments\Client\Model\CreateCommentParams(); // \FastComments\Client\Model\CreateCommentParams
$is_live = True; // bool
$do_spam_check = True; // bool
$send_emails = True; // bool
$populate_notifications = True; // bool

try {
    $result = $apiInstance->saveComment($tenant_id, $create_comment_params, $is_live, $do_spam_check, $send_emails, $populate_notifications);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->saveComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]