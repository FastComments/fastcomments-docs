## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| includeByUserIdAndEmail | boolean | query | Ne |  |
| includeByIP | boolean | query | Ne |  |
| includeByEmailDomain | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`PreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PreBanSummary.php)

## Primjer

[inline-code-attrs-start title = 'Primjer getPreBanSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, kao zadani klijent će se koristiti `GuzzleHttp\Client`.
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