Tidligere kommentatorer på siden, som IKKE er online i øjeblikket. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at gengive en "Medlemmer"-sektion.
Cursor-paginering på commenterName: serveren gennemgår det delvise {tenantId, urlId, commenterName}-indeks fra afterName fremad via $gt, uden $skip-omkostning.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | sti | Ja |  |
| urlId | string | forespørgsel | Ja | Side-URL-identifikator (renset på serversiden). |
| afterName | string | forespørgsel | Nej | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | forespørgsel | Nej | Cursor-tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så indlæg med samme navn ikke bliver udeladt. |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers-eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, giv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Side-URL-identifikator (renset på serversiden).
$after_name = 'after_name_example'; // string | Cursor: angiv nextAfterName fra det forrige svar.
$after_user_id = 'after_user_id_example'; // string | Cursor-tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så indlæg med samme navn ikke bliver udeladt.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]