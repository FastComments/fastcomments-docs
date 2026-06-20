## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlId | string | query | Hayır | Mevcut sayfanın aboneliğinin olup olmadığını belirlemek için kullanılır. |
| pageSize | integer | query | Hayır |  |
| afterId | string | query | Hayır |  |
| includeContext | boolean | query | Hayır |  |
| afterCreatedAt | integer | query | Hayır |  |
| unreadOnly | boolean | query | Hayır |  |
| dmOnly | boolean | query | Hayır |  |
| noDm | boolean | query | Hayır |  |
| includeTranslations | boolean | query | Hayır |  |
| includeTenantNotifications | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Dönen değer: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetMyNotificationsResponse.php)

## Örnek

[inline-code-attrs-start title = 'getUserNotifications Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir http istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Mevcut sayfanın aboneliğinin olup olmadığını belirlemek için kullanılır.
$page_size = 56; // int
$after_id = 'after_id_example'; // string
$include_context = True; // bool
$after_created_at = 56; // int
$unread_only = True; // bool
$dm_only = True; // bool
$no_dm = True; // bool
$include_translations = True; // bool
$include_tenant_notifications = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserNotifications($tenant_id, $url_id, $page_size, $after_id, $include_context, $after_created_at, $unread_only, $dm_only, $no_dm, $include_translations, $include_tenant_notifications, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]