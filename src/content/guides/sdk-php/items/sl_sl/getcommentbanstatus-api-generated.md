## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| commentId | string | pot | Da |  |
| sso | string | poizvedba | Ne |  |

## Odgovor

Vrne: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentBanStatusResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getCommentBanStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To ni obvezno, kot privzeti bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // niz
$sso = 'sso_example'; // niz

try {
    $result = $apiInstance->getCommentBanStatus($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCommentBanStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]