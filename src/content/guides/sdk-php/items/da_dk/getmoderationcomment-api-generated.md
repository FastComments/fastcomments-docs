## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| includeEmail | boolean | query | Nej |  |
| includeIP | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICommentResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getModerationComment Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en tilpasset HTTP-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$options = [
    'include_email' => True, // bool
    'include_ip' => True, // bool
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getModerationComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getModerationComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---