## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| direction | string | query | Ne |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odziv

Vrne: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## Primer

[inline-code-attrs-start title = 'postVote Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\ModerationApi(
//     // Če želite uporabiti svoj HTTP odjemalec, prenesite svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
//     // To je neobvezno, kot privzeto bo uporabljen `GuzzleHttp\Client`.
//     new GuzzleHttp\Client()
// );

$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$options = [
    'direction' => 'direction_example', // string
    'broadcast_id' => 'broadcast_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->postVote($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Izjema pri klicu ModerationApi->postVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]