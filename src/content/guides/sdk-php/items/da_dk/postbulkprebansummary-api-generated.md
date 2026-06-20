## Parametre

| Name | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Nej |  |
| includeByIP | boolean | query | Nej |  |
| includeByEmailDomain | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkPreBanSummary.php)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på postBulkPreBanSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du sende din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$bulk_pre_ban_params = new \FastComments\Client\Model\BulkPreBanParams(); // \FastComments\Client\Model\BulkPreBanParams
$include_by_user_id_and_email = True; // bool
$include_by_ip = True; // bool
$include_by_email_domain = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postBulkPreBanSummary($bulk_pre_ban_params, $include_by_user_id_and_email, $include_by_ip, $include_by_email_domain, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBulkPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]