Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор на URL на страницата (почистен от страна на сървъра). |
| afterName | string | query | No | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | No | Турен брейк за курсора: подайте nextAfterUserId от предишния отговор. Задължително, когато afterName е зададен, за да не се изпускат записи при равенство на имената. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор на URL на страницата (почистен от страна на сървъра).
$options = [
    'after_name' => 'after_name_example', // string | Курсор: подайте nextAfterName от предишния отговор.
    'after_user_id' => 'after_user_id_example', // string | Турен брейк за курсора: подайте nextAfterUserId от предишния отговор. Задължително, когато afterName е зададен, за да не се изпускат записи при равенство на имената.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]