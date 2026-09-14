FastComments é um servidor de autorização OAuth 2.1. Uma aplicação pode obter um token que está vinculado a uma conta FastComments e usá-lo em todos os endpoints deste guia no lugar de uma chave de API. É assim que o aplicativo Zapier, o servidor MCP e outras integrações de terceiros se conectam.

Os tokens são emitidos através do fluxo de código de autorização com PKCE. Não há concessão de credenciais de cliente nem concessão implícita.

### Discovery

Os locais dos endpoints, concessões suportadas e métodos de autenticação são publicados na URL padrão de metadados:

[inline-code-attrs-start title = 'Metadados do Servidor de Autorização'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Os endpoints que ele descreve:

[inline-code-attrs-start title = 'Endpoints OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Contas na região da UE usam `https://eu.fastcomments.com` como emissor, com os mesmos caminhos.

### Registering a client

Um cliente precisa de um `client_id` e de um `redirect_uri` registrado antes de iniciar o fluxo. Existem duas maneiras de obter um:

- **Registro Dinâmico de Cliente.** `POST /oauth/register` com um corpo JSON conforme RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). A resposta contém o `client_id` e, para clientes confidenciais, o `client_secret`. O registro não requer autenticação e tem limite de taxa por IP.
- **Documento de Metadados de ID de Cliente.** O cliente usa uma URL `https` que controla como seu `client_id`. FastComments busca essa URL e lê os mesmos campos de metadados dela. Nenhuma chamada de registro é necessária.

Aplicações parceiras listadas no painel do FastComments, como Zapier, são registradas diretamente pelo FastComments. Entre em contato com o suporte se você estiver criando uma listagem de marketplace e precisar de um cliente de primeira parte.

### Scopes

[inline-code-attrs-start title = 'Escopos'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Uma solicitação que não pede nenhum escopo recebe ambos. O usuário vê os escopos solicitados na página de consentimento. Uma solicitação por um escopo diferente desses dois falha com `invalid_scope`.

### Step 1 - Authorization request

Envie o navegador do usuário para o endpoint de autorização. PKCE com o método `S256` é obrigatório para todo cliente.

[inline-code-attrs-start title = 'Solicitação de Autorização'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = 'Parâmetros da Solicitação de Autorização'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Must exactly match one of the client's registered redirect URIs. **/
    redirect_uri: string
    /** Space separated. Omit to request both scopes. **/
    scope?: 'read' | 'write' | 'read write'
    /** Returned unchanged on the redirect. Use it to bind the callback to the session that started the flow. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optional RFC 8707 resource indicator. If sent, the same value must be sent to the token endpoint. **/
    resource?: string
}
[inline-code-end]

O usuário faz login no FastComments, se necessário, e vê uma página de consentimento nomeando sua aplicação, a conta à qual será conectada e os escopos solicitados. O usuário deve possuir a permissão **API Admin** nessa conta; qualquer outra pessoa vê um erro de permissão em vez do formulário de consentimento. Aprovar redireciona o navegador para seu `redirect_uri` com `code` e `state`. Negar redireciona com `error=access_denied`.

O código de autorização é válido por 10 minutos e pode ser trocado uma única vez. Uma segunda troca do mesmo código revoga todos os tokens produzidos na primeira troca.

### Step 2 - Token request

Troque o código por tokens. O corpo é codificado como formulário. Clientes confidenciais autenticam com `client_secret_basic` (HTTP Basic) ou `client_secret_post` (segredo no corpo). Clientes públicos enviam apenas `client_id`.

[inline-code-attrs-start title = 'Exemplo cURL de Solicitação de Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'Corpo da Solicitação de Token (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Confidential clients only. May be sent as HTTP Basic auth instead. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Must match the authorization request when sent. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Token'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefixed fcat_. Valid for one hour. **/
    access_token: string
    token_type: 'bearer'
    /** Seconds until the access token expires. 3600. **/
    expires_in: number
    /** Prefixed fcrt_. Valid for 30 days from issue. **/
    refresh_token: string
    /** Space separated scopes granted. **/
    scope: string
}
[inline-code-end]

Erros seguem a RFC 6749: um corpo JSON com `error` e `error_description`, HTTP 400 para `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` e `unsupported_grant_type`, HTTP 401 para `invalid_client`, HTTP 429 quando há limite de taxa.

### Step 3 - Calling the API

Envie o token de acesso como token bearer. O tenant é implícito no token, portanto `tenantId` é opcional. Quando fornecido, deve corresponder ao token ou a solicitação falhará.

[inline-code-attrs-start title = 'Exemplo cURL de Token Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` retorna o tenant, o usuário autorizador e os escopos concedidos, o que o torna a chamada correta para um teste de conexão. Uma solicitação com um token expirado ou revogado recebe HTTP 401. Uma solicitação cujo método requer um escopo que o token não possui recebe HTTP 403.

### Step 4 - Refreshing

[inline-code-attrs-start title = 'Exemplo cURL de Solicitação de Atualização'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Corpo da Solicitação de Token (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Optional. Narrows to a subset of the scopes originally granted. **/
    scope?: string
    resource?: string
}
[inline-code-end]

A resposta tem o mesmo formato da troca de código. Tokens de atualização giram: cada atualização retorna um novo `refresh_token` e revoga o antigo após uma janela de 30 segundos para solicitações concorrentes. Apresentar um token de atualização que foi girado há mais de 30 segundos é tratado como replay e revoga toda a concessão. Aplicações parceiras registradas pelo FastComments são isentas da rotação e recebem o mesmo refresh token com sua validade estendida por mais 30 dias.

Uma atualização também verifica novamente se o usuário autorizador ainda possui API Admin na conta. Caso não, a concessão é revogada e a resposta é `invalid_grant`.

### Revocation

[inline-code-attrs-start title = 'Exemplo cURL de Solicitação de Revogação'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Revogar um refresh token revoga todos os tokens de acesso emitidos a partir da mesma concessão. Revogar um token de acesso revoga apenas esse token. O endpoint retorna HTTP 200 com um objeto JSON vazio, independentemente de o token ter sido encontrado, conforme RFC 7009.

Os usuários também podem revogar uma conexão a partir de **Aplicativos Conectados** no painel do FastComments. Todos os tokens para essa aplicação deixam de funcionar imediatamente.