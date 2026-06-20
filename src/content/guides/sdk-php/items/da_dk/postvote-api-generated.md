## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| direction | string | query | Nej |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## Eksempel

[inline-code-attrs-start title = 'postVote Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en tilpasset HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` bruges som standard.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // streng
$direction = 'direction_example'; // streng
$sso = 'sso_example'; // streng

try {
    $result = $apiInstance->postVote($comment_id, $direction, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]