## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## Primer

[inline-code-attrs-start title = 'resetUserNotificationCount Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\PublicApi(
//     // Če želite uporabiti svoj HTTP odjemalec, prenesite svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
//     // To je neobvezno, kot privzeto bo uporabljen `GuzzleHttp\Client`.
//     new GuzzleHttp\Client()
// );

$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti svoj HTTP odjemalec, prenesite svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$sso = 'sso_example'; // string


try {
    $result = $apiInstance->resetUserNotificationCount($tenant_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]