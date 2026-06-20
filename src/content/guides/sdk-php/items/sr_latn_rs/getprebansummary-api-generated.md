## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | putanja | Da |  |
| includeByUserIdAndEmail | boolean | upit | Ne |  |
| includeByIP | boolean | upit | Ne |  |
| includeByEmailDomain | boolean | upit | Ne |  |
| sso | string | upit | Ne |  |

## Odgovor

Vraća: [`PreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PreBanSummary.php)

## Primer

[inline-code-attrs-start title = 'Primer getPreBanSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će biti korišćen kao podrazumevani.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$include_by_user_id_and_email = True; // bool
$include_by_ip = True; // bool
$include_by_email_domain = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getPreBanSummary($comment_id, $include_by_user_id_and_email, $include_by_ip, $include_by_email_domain, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]