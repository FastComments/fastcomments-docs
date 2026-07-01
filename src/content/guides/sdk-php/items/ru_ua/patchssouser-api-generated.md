## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| updateComments | boolean | query | Ні |  |

## Відповідь

Повертає: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchSSOUserAPIResponse.php)

## Приклад

[inline-code-attrs-start title = 'patchSSOUser Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштуйте авторизацію API-ключа: api_key
// Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API-ключа, за потреби
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це опціонально, `GuzzleHttp\Client` буде використаний за замовчуванням.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_apisso_user_data = new \FastComments\Client\Model\UpdateAPISSOUserData(); // \FastComments\Client\Model\UpdateAPISSOUserData
$update_comments = True; // bool


try {
    $result = $apiInstance->patchSSOUser($tenant_id, $id, $update_apisso_user_data, $update_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]