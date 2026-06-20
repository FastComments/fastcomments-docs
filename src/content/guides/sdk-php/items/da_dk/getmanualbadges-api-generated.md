## Parametre

| Name | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantManualBadgesResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getManualBadges Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, angiv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getManualBadges($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getManualBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---