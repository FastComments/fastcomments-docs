Tidligere kommentatorer på siden, som IKKE er online i øjeblikket. Sorteret efter displayName.
Brug denne, efter du har brugt /users/online op, til at rendere et "Members"-afsnit.
Cursor-paginering på commenterName: serveren gennemløber det partielle {tenantId, urlId, commenterName}
indeks fra afterName fremad via $gt, uden $skip-omkostning.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Side-URL-identifikator (renset server-side). |
| afterName | string | query | Nej | Cursor: send nextAfterName fra det forrige svar. |
| afterUserId | string | query | Nej | Cursor tiebreaker: send nextAfterUserId fra det forrige svar. Påkrævet, når afterName er angivet, så navn-ligheder ikke dropper poster. |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret http-klient, send din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` bruges som standard.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Side-URL-identifikator (renset server-side).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: send nextAfterName fra det forrige svar.
    'after_user_id' => 'after_user_id_example', // string | Cursor tiebreaker: send nextAfterUserId fra det forrige svar. Påkrævet, når afterName er angivet, så navn-ligheder ikke dropper poster.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]