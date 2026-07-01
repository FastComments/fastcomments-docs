## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|-----------|------|
| tenantId | string | path | Da |  |
| commentId | string | path | Da |  |
| voteId | string | path | Da |  |
| urlId | string | query | Da |  |
| broadcastId | string | query | Da |  |
| editKey | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## Primer

[inline-code-attrs-start title = 'deleteCommentVote Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, `GuzzleHttp\Client` bo privzeto uporabljen.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // niz
$comment_id = 'comment_id_example'; // niz
$vote_id = 'vote_id_example'; // niz
$url_id = 'url_id_example'; // niz
$broadcast_id = 'broadcast_id_example'; // niz
$options = [
    'edit_key' => 'edit_key_example', // niz
    'sso' => 'sso_example', // niz
];


try {
    $result = $apiInstance->deleteCommentVote($tenant_id, $comment_id, $vote_id, $url_id, $broadcast_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---