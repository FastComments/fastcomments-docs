Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Side‑URL‑identifikator (rengjort på server‑siden). |
| afterName | string | query | Nej | Cursor: send nextAfterName fra den forrige respons. |
| afterUserId | string | query | Nej | Cursor‑tiebreaker: send nextAfterUserId fra den forrige respons. Påkrævet når afterName er angivet, så navne‑ties ikke fjerner poster. |

## Respons

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en tilpasset HTTP‑klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Side‑URL‑identifikator (rengjort på server‑siden).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: send nextAfterName fra den forrige respons.
    'after_user_id' => 'after_user_id_example', // string | Cursor‑tiebreaker: send nextAfterUserId fra den forrige respons. Påkrævet når afterName er angivet, så navne‑ties ikke fjerner poster.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]