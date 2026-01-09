## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| voteId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| broadcastId | string | query | Ja |  |
| editKey | string | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentVote200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'deleteCommentVote Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef dan uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt als standaard gebruikt.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$vote_id = 'vote_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$edit_key = 'edit_key_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteCommentVote($tenant_id, $comment_id, $vote_id, $url_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]