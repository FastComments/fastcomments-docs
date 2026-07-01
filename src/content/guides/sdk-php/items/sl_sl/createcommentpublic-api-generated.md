## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | pot | Da |  |
| urlId | string | poizvedba | Da |  |
| broadcastId | string | poizvedba | Da |  |
| sessionId | string | poizvedba | Ne |  |
| sso | string | poizvedba | Ne |  |

## Odgovor

Vrne: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SaveCommentsResponseWithPresence.php)

## Primer

[inline-code-attrs-start title = 'Primer createCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti prilagojeni http odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$comment_data = new \FastComments\Client\Model\CommentData(); // \FastComments\Client\Model\CommentData
$options = [
    'session_id' => 'session_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->createCommentPublic($tenant_id, $url_id, $broadcast_id, $comment_data, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]