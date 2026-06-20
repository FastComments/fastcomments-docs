Eerdere commentatoren op de pagina die momenteel NIET online zijn. Gesorteerd op displayName.
Gebruik dit nadat /users/online is uitgeput om een "Leden"-sectie weer te geven.
Cursor-paginering op commenterName: de server doorloopt de gedeeltelijke {tenantId, urlId, commenterName}-index vanaf afterName vooruit via $gt, zonder $skip-kosten.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina-URL-identificatie (schoongemaakt aan de serverzijde). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door uit het vorige antwoord. |
| afterUserId | string | query | Nee | Tiebreaker voor cursor: geef nextAfterUserId door uit het vorige antwoord. Vereist wanneer afterName is ingesteld zodat bij naamgelijkheden geen items verloren gaan. |

## Antwoord

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Pagina-URL-identificatie (schoongemaakt aan de serverzijde).
$after_name = 'after_name_example'; // string | Cursor: geef nextAfterName door uit het vorige antwoord.
$after_user_id = 'after_user_id_example'; // string | Tiebreaker voor cursor: geef nextAfterUserId door uit het vorige antwoord. Vereist wanneer afterName is ingesteld zodat bij naamgelijkheden geen items verloren gaan.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---