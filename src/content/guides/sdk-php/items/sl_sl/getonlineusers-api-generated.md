Trenutno prisotni gledalci strani: osebe, katerih websocket seja je trenutno naročena na to stran.
Vrne anonCount + totalCount (naročniki po celotni sobi, vključno z anonimnimi gledalci, ki jih ne navajamo).

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL strani (očiščen na strežniku). |
| afterName | string | query | Ne | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Ločilo pri izenačenju: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljena, da vnosi z enakim imenom ne bi bili izpuščeni. |

## Odgovor

Vrne: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To ni obvezno; privzeto se uporabi `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL strani (očiščen na strežniku).
$after_name = 'after_name_example'; // string | Kazalec: posredujte nextAfterName iz prejšnjega odgovora.
$after_user_id = 'after_user_id_example'; // string | Ločilo pri izenačenju: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljena, da vnosi z enakim imenom ne bi bili izpuščeni.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]