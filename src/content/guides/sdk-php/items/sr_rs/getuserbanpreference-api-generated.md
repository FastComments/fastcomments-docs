## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIModerateGetUserBanPreferencesResponse.php)

## Primer

[inline-code-attrs-start title = 'getUserBanPreference Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, koristiće se `GuzzleHttp\Client` kao podrazumevani.
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserBanPreference($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserBanPreference: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---