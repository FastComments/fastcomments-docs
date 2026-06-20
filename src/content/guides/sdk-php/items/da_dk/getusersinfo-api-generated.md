Bulk-brugerinformation for en tenant. Givet userIds returneres visningsoplysninger fra User / SSOUser.
Bruges af comment widget til at berige brugere, der lige er dukket op via en tilstedeværelsesbegivenhed.
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler maskeres).

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| ids | string | query | Ja | Komma-separerede userIds. |

## Svar

Returnerer: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getUsersInfo Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, angiv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Kommaseparerede userIds.

try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]