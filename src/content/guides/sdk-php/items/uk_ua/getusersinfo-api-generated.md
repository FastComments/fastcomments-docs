Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Used by the comment widget to enrich users that just appeared via a presence event.  
No page context: privacy is enforced uniformly (private profiles are masked).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| ids | string | query | Так | Кома‑розділені userIds. |

## Відповідь

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\PublicApi(
//     // Якщо ви хочете використовувати власний HTTP‑клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
//     // Це не обов’язково, за замовчуванням буде використано `GuzzleHttp\Client`.
//     new GuzzleHttp\Client()
// );

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Кома‑розділені userIds.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]