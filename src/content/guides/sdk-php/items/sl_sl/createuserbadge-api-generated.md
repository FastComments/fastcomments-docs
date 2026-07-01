## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Odgovor

Vrne: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APICreateUserBadgeResponse.php)

## Primer

[inline-code-attrs-start title = 'createUserBadge Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Nastavite avtentikacijo prek API ključa: api_key
// Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
// Če želite uporabiti prilagojeni HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
// To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.

$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti prilagojeni HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$create_user_badge_params = new \FastComments\Client\Model\CreateUserBadgeParams(); // \FastComments\Client\Model\CreateUserBadgeParams


try {
    $result = $apiInstance->createUserBadge($tenant_id, $create_user_badge_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]