## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nee | Gebruikt om te bepalen of de huidige pagina geabonneerd is. |
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

Retourneert: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetMyNotificationsResponse.php)

## Example

[inline-code-attrs-start title = 'getUserNotifications Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als je een aangepaste http-client wilt gebruiken, geef je jouw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'url_id' => 'url_id_example', // string | Gebruikt om te bepalen of de huidige pagina geabonneerd is.
    'page_size' => 56, // int
    'after_id' => 'after_id_example', // string
    'include_context' => True, // bool
    'after_created_at' => 56, // int
    'unread_only' => True, // bool
    'dm_only' => True, // bool
    'no_dm' => True, // bool
    'include_translations' => True, // bool
    'include_tenant_notifications' => True, // bool
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getUserNotifications($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]