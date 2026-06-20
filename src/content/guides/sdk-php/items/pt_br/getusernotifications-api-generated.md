## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Não | Usado para determinar se a página atual está inscrita. |
| pageSize | integer | query | Não |  |
| afterId | string | query | Não |  |
| includeContext | boolean | query | Não |  |
| afterCreatedAt | integer | query | Não |  |
| unreadOnly | boolean | query | Não |  |
| dmOnly | boolean | query | Não |  |
| noDm | boolean | query | Não |  |
| includeTranslations | boolean | query | Não |  |
| includeTenantNotifications | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetMyNotificationsResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Usado para determinar se a página atual está inscrita.
$page_size = 56; // int
$after_id = 'after_id_example'; // string
$include_context = True; // bool
$after_created_at = 56; // int
$unread_only = True; // bool
$dm_only = True; // bool
$no_dm = True; // bool
$include_translations = True; // bool
$include_tenant_notifications = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserNotifications($tenant_id, $url_id, $page_size, $after_id, $include_context, $after_created_at, $unread_only, $dm_only, $no_dm, $include_translations, $include_tenant_notifications, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]