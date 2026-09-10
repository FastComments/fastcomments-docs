### A API FastComments

FastComments fornece uma API para interagir com muitos recursos. Crie integrações com nossa plataforma ou até mesmo crie seus próprios clientes!

Nesta documentação, você encontrará todos os recursos suportados pela API documentados com seus tipos de requisição e resposta.

Para clientes Enterprise, todo acesso à API é registrado no Log de Auditoria.

### SDKs Gerados

FastComments agora gera uma [Especificação de API](https://fastcomments.com/js/swagger.json) a partir do nosso código (ainda não está completa, mas inclui muitas APIs).

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

A API é autenticada passando sua [chave de API](https://fastcomments.com/auth/my-account/api-secret) como um cabeçalho `X-API-KEY` ou como parâmetro de consulta `API_KEY`. Você também precisará do seu `tenantId` para fazer chamadas à API. Ele pode ser obtido na mesma página da sua chave de API.

### Nota de Segurança

Essas rotas devem ser chamadas a partir de um **servidor**. __NÃO__ as chame a partir de um navegador. Fazer isso exporá sua chave de API – isso dará acesso total à sua conta a qualquer pessoa que possa ver o código-fonte de uma página!

#### Opção de Autenticação Um - Cabeçalhos

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opção de Autenticação Dois - Parâmetros de Consulta

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Opção de Autenticação Três - Token OAuth Bearer

- Header: `Authorization: Bearer fcat_...`

Aplicações que se conectam através do [servidor MCP](https://docs.fastcomments.com/guide-llm-kit.html) obtêm um token via OAuth em vez de uma chave de API. Esse token funciona em todos os endpoints aqui. O tenant é implícito pelo token, portanto `tenantId` é opcional, mas deve corresponder ao token quando fornecido. Requisições `GET` precisam do escopo `read` e todo outro método precisa do escopo `write`. A descoberta começa em `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Lendo Suas Próprias Escritas

FastComments oferece disponibilidade Active-Active. Requisições do seu data center são roteadas para [o ponto de presença mais próximo](https://sophon.fastcomments.com/) do seu. Isso é automático e normalmente você pode observar a semântica de leitura após escrita. Se quiser garantir a leitura de suas próprias escritas, pode fixar suas requisições a uma região específica usando essa região como host da API (embora isso geralmente não seja necessário para a maioria das integrações):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Observe que, se fizer isso, pode querer definir um fallback, pois já depreciamos nós de ponto de entrada no passado e usamos novos nomes para a troca.