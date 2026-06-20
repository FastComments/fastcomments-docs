## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| voteId | string | path | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## Eksempel

[inline-code-attrs-start title = 'deleteModerationVote Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, så giv din klient, der implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$vote_id = 'vote_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteModerationVote($comment_id, $vote_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->deleteModerationVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---