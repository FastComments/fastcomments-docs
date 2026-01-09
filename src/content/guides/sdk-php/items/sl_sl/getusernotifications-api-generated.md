## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| pageSize | integer | query | Ne |  |
| afterId | string | query | Ne |  |
| includeContext | boolean | query | Ne |  |
| afterCreatedAt | integer | query | Ne |  |
| unreadOnly | boolean | query | Ne |  |
| dmOnly | boolean | query | Ne |  |
| noDm | boolean | query | Ne |  |
| includeTranslations | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserNotifications200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer getUserNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, kot privzeti bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$page_size = 56; // int
$after_id = 'after_id_example'; // string
$include_context = True; // bool
$after_created_at = 56; // int
$unread_only = True; // bool
$dm_only = True; // bool
$no_dm = True; // bool
$include_translations = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserNotifications($tenant_id, $page_size, $after_id, $include_context, $after_created_at, $unread_only, $dm_only, $no_dm, $include_translations, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]