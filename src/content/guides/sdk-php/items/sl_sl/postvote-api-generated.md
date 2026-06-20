## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| direction | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer postVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Če želite uporabiti lasten HTTP odjemalec, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je opcijsko, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // niz
$direction = 'direction_example'; // niz
$sso = 'sso_example'; // niz

try {
    $result = $apiInstance->postVote($comment_id, $direction, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]