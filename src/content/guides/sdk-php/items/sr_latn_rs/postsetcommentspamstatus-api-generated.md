## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| spam | boolean | query | Ne |  |
| permNotSpam | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Primer

[inline-code-attrs-start title = 'postSetCommentSpamStatus Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, biće korišćen `GuzzleHttp\Client` kao podrazumevani.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$spam = True; // bool
$perm_not_spam = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postSetCommentSpamStatus($comment_id, $spam, $perm_not_spam, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentSpamStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]