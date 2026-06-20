O SDK expõe três classes cliente de API:

- **`DefaultApi`** — métodos autenticados por chave de API para uso no lado do servidor. Configure uma chave de API como mostrado em [Introdução](#getting-started-readme-generated).
- **`PublicApi`** — métodos públicos que não requerem uma chave de API, seguros para serem chamados a partir de navegadores e aplicativos móveis.
- **`ModerationApi`** — métodos para o painel de moderação: listagem, contagem, busca, registro e exportação de comentários; ações de moderação (remover/restaurar, sinalizar, definir status de revisão/spam/aprovação, votos, reabrir/fechar thread); banimentos (banir de comentar, desfazer, resumos pré-banimento, status de banimento e preferências, contagens de usuários banidos); e distintivos & confiança (atribuir/remover distintivo, distintivos manuais, obter/definir fator de confiança, perfil interno do usuário). Cada método de `ModerationApi` aceita um parâmetro `$sso` para autenticar o moderador atuante via SSO.

### Usando PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Public methods do not require an API key.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Usando ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - payload SSO que autentica o moderador

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```