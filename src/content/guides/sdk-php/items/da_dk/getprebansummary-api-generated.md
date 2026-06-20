## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| includeByUserIdAndEmail | boolean | query | Nej |  |
| includeByIP | boolean | query | Nej |  |
| includeByEmailDomain | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`PreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PreBanSummary.php)

## Eksempel

[inline-code-attrs-start title = 'getPreBanSummary Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // streng
$include_by_user_id_and_email = True; // boolsk
$include_by_ip = True; // boolsk
$include_by_email_domain = True; // boolsk
$sso = 'sso_example'; // streng

try {
    $result = $apiInstance->getPreBanSummary($comment_id, $include_by_user_id_and_email, $include_by_ip, $include_by_email_domain, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]