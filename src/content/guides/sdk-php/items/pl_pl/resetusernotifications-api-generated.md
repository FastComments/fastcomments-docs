---
## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| afterId | string | query | Nie |  |
| afterCreatedAt | integer | query | Nie |  |
| unreadOnly | boolean | query | Nie |  |
| dmOnly | boolean | query | Nie |  |
| noDm | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład resetUserNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, jako domyślny zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$after_id = 'after_id_example'; // string
$after_created_at = 56; // int
$unread_only = True; // bool
$dm_only = True; // bool
$no_dm = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->resetUserNotifications($tenant_id, $after_id, $after_created_at, $unread_only, $dm_only, $no_dm, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---