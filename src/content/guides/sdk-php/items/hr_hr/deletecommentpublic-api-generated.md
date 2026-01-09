## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| commentId | string | path | Da |  |
| broadcastId | string | query | Da |  |
| editKey | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentPublic200Response.php)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, kao zadani će se koristiti `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$edit_key = 'edit_key_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteCommentPublic($tenant_id, $comment_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]