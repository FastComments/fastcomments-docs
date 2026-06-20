## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserInternalProfileResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getUserInternalProfile'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će se koristiti kao podrazumevani.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserInternalProfile($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserInternalProfile: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---