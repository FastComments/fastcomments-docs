### A API do FastComments

A FastComments fornece uma API para interagir com diversos recursos. Crie integrações com nossa plataforma ou até mesmo construa seus próprios clientes!

Nesta documentação, você encontrará todos os recursos suportados pela API documentados com seus tipos de requisição e resposta.

Para clientes Enterprise, todo acesso à API é registrado no Registro de Auditoria.

### SDKs Gerados

A FastComments agora gera uma [Especificação da API](https://fastcomments.com/js/swagger.json) a partir do nosso código (isso ainda não está completo, mas inclui muitas APIs).

Agora também temos SDKs para linguagens populares:

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

A API é autenticada passando sua [api key](https://fastcomments.com/auth/my-account/api-secret) como um header `X-API-KEY` ou parâmetro de query `API_KEY`. Você também precisará do seu `tenantId` para fazer chamadas à API. Isto pode ser recuperado na mesma página da sua api key.

### Nota de Segurança

Essas rotas devem ser chamadas a partir de um **server**. __DO NOT__ chame-as a partir de um navegador. Fazer isso exporá sua API key — isso dará acesso total à sua conta a qualquer pessoa que consiga ver o código-fonte de uma página!

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Lendo suas próprias gravações

As requisições do seu datacenter são roteadas para [o ponto de presença mais próximo](https://sophon.fastcomments.com/) ao seu. Isso é automático e, normalmente, você pode observar a semântica de leitura-após-gravação. Se quiser ter certeza de ler suas próprias gravações, você pode fixar suas requisições a uma determinada região usando essa região como host da API (no entanto, isso geralmente não é necessário para a maioria das integrações):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Observe que, se fizer isso, pode ser interessante definir um mecanismo de fallback, pois descontinuamos nós de entrada no passado e usamos novos nomes para a transição.