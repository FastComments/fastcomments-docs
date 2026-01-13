### A API do FastComments

FastComments fornece uma API para interagir com muitos recursos. Crie integrações com nossa plataforma, ou até construa seus próprios clientes!

Nesta documentação, você encontrará todos os recursos suportados pela API documentados com seus tipos de requisição e resposta.

Para clientes Enterprise, todo o acesso à API é registrado no Registro de Auditoria.

### SDKs Gerados

O FastComments agora gera uma [Especificação da API](https://fastcomments.com/js/swagger.json) a partir do nosso código (isso ainda não está completo, mas inclui muitas APIs).

Também agora temos SDKs para linguagens populares:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Autenticação

A API é autenticada passando sua [chave da API](https://fastcomments.com/auth/my-account/api-secret) como um cabeçalho `X-API-KEY` ou parâmetro de consulta `API_KEY`. Você também precisará do seu `tenantId` para fazer chamadas à API. Isso pode ser recuperado na mesma página que sua chave da API.

### Nota de Segurança

Essas rotas devem ser chamadas a partir de um **servidor**. __NÃO__ as chame a partir de um navegador. Fazer isso expõe sua chave da API — isso fornecerá acesso total à sua conta para qualquer pessoa que possa ver o código-fonte de uma página!

#### Opção de Autenticação Um - Cabeçalhos

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opção de Autenticação Dois - Parâmetros de Consulta

- Parâmetro de Consulta: `API_KEY`
- Parâmetro de Consulta: `tenantId`

---