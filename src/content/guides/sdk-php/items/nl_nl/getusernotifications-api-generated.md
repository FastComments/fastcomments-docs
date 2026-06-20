## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nee | Wordt gebruikt om te bepalen of de huidige pagina is geabonneerd. |
| pageSize | integer | query | Nee |  |
| afterId | string | query | Nee |  |
| includeContext | boolean | query | Nee |  |
| afterCreatedAt | integer | query | Nee |  |
| unreadOnly | boolean | query | Nee |  |
| dmOnly | boolean | query | Nee |  |
| noDm | boolean | query | Nee |  |
| includeTranslations | boolean | query | Nee |  |
| includeTenantNotifications | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Geeft terug: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetMyNotificationsResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getUserNotifications Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef dan uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt gebruikt als standaard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Wordt gebruikt om te bepalen of de huidige pagina is geabonneerd.
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