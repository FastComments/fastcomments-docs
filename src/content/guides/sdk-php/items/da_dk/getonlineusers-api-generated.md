Aktuelt online seere af en side: personer hvis websocket-session er tilmeldt siden lige nu.
Returnerer anonCount + totalCount (abonnenter for hele rummet, inklusive anonyme seere, som vi ikke opregner).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side-URL-identifikator (renset på serversiden). |
| afterName | string | query | No | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | query | No | Tiebreaker for cursor: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så rækker med samme navn ikke bliver udeladt. |

## Response

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du ønsker at bruge en brugerdefineret http-klient, så videregiv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Side-URL-identifikator (renset på serversiden).
$after_name = 'after_name_example'; // string | Cursor: angiv nextAfterName fra det forrige svar.
$after_user_id = 'after_user_id_example'; // string | Tiebreaker for cursor: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så rækker med samme navn ikke bliver udeladt.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---