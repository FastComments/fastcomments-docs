Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Seiten-URL-Identifikator (serverseitig bereinigt). |
| afterName | string | query | No | Cursor: übergebe nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | No | Cursor-Tiebreaker: übergebe nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge entfernen. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'Beispiel für getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Seiten-URL-Identifikator (serverseitig bereinigt).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: übergebe nextAfterName aus der vorherigen Antwort.
    'after_user_id' => 'after_user_id_example', // string | Cursor-Tiebreaker: übergebe nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge entfernen.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]