## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| badgeId | string | query | Ne |  |
| type | number | query | Ne |  |
| displayedOnComments | boolean | query | Ne |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrača: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadges200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer getUserBadges'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtentikacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodnje, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti svoj HTTP odjemalec, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je izbirno, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$badge_id = 'badge_id_example'; // string
$type = 3.4; // float
$displayed_on_comments = True; // bool
$limit = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getUserBadges($tenant_id, $user_id, $badge_id, $type, $displayed_on_comments, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]