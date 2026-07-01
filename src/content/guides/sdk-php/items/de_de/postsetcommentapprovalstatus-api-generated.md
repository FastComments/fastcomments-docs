## Parameter

| Name | Type | Location | Erforderlich | Beschreibung |
|------|------|----------|--------------|--------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| approved | boolean | query | Nein |  |
| broadcastId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetCommentApprovedResponse.php)

## Beispiel

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // Zeichenkette
$comment_id = 'comment_id_example'; // Zeichenkette
$options = [
    'approved' => True, // bool
    'broadcast_id' => 'broadcast_id_example', // Zeichenkette
    'sso' => 'sso_example', // Zeichenkette
];


try {
    $result = $apiInstance->postSetCommentApprovalStatus($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentApprovalStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]