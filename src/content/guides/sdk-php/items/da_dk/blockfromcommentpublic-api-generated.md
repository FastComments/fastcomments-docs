## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Svar

Returnerer: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BlockFromCommentPublic200Response.php)

## Eksempel

[inline-code-attrs-start title = 'blockFromCommentPublic Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$public_block_from_comment_params = new \FastComments\Client\Model\PublicBlockFromCommentParams(); // \FastComments\Client\Model\PublicBlockFromCommentParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->blockFromCommentPublic($tenant_id, $comment_id, $public_block_from_comment_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->blockFromCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]