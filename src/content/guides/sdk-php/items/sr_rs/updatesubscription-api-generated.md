## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |

## Одговор

Враћа: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateSubscriptionAPIResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример updateSubscription'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуришите ауторизацију API кључа: api_key
// Одкоментаришите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевано.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // стринг
$id = 'id_example'; // стринг
$update_api_user_subscription_data = new \FastComments\Client\Model\UpdateAPIUserSubscriptionData(); // \FastComments\Client\Model\UpdateAPIUserSubscriptionData
$user_id = 'user_id_example'; // стринг


try {
    $result = $apiInstance->updateSubscription($tenant_id, $id, $update_api_user_subscription_data, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateSubscription: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---