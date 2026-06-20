## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentBanStatusResponse.php)

## Primjer

[inline-code-attrs-start title = 'getCommentBanStatus Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, po defaultu će se koristiti `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentBanStatus($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCommentBanStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]