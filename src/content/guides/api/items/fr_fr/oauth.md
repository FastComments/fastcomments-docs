FastComments est un serveur d'autorisation OAuth 2.1. Une application peut obtenir un jeton lié à un compte FastComments et l'utiliser sur chaque point de terminaison de ce guide à la place d'une clé API. C'est ainsi que l'application Zapier, le serveur MCP et d'autres intégrations tierces se connectent.

Les jetons sont émis via le flux de code d'autorisation avec PKCE. Il n'existe pas de flux client credentials ou implicit grant.

### Découverte

Les emplacements des points de terminaison, les subventions prises en charge et les méthodes d'authentification sont publiés à l'URL de métadonnées standard :

[inline-code-attrs-start title = 'Métadonnées du serveur d\'autorisation'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Les points de terminaison qu'il décrit :

[inline-code-attrs-start title = 'Points de terminaison OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Les comptes de la région UE utilisent `https://eu.fastcomments.com` comme émetteur, avec les mêmes chemins.

### Enregistrement d'un client

Un client a besoin d'un `client_id` et d'un `redirect_uri` enregistré avant de pouvoir démarrer le flux. Il existe deux façons d'en obtenir un :

- **Enregistrement dynamique de client.** `POST /oauth/register` avec un corps JSON selon la RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). La réponse contient le `client_id` et, pour les clients confidentiels, le `client_secret`. L'enregistrement n'est pas authentifié et est limité en taux par IP.
- **Document de métadonnées d'ID client.** Le client utilise une URL `https` qu'il contrôle comme son `client_id`. FastComments récupère cette URL et lit les mêmes champs de métadonnées. Aucun appel d'enregistrement n'est nécessaire.

Les applications partenaires répertoriées dans le tableau de bord FastComments, comme Zapier, sont enregistrées directement par FastComments. Contactez le support si vous créez une liste de place de marché et avez besoin d'un client de première partie.

### Portées

[inline-code-attrs-start title = 'Portées'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Une requête qui ne demande aucune portée reçoit les deux. L'utilisateur voit les portées demandées sur la page de consentement. Une requête pour une portée autre que ces deux échoue avec `invalid_scope`.

### Étape 1 - Requête d'autorisation

Envoyez le navigateur de l'utilisateur vers le point de terminaison d'autorisation. PKCE avec la méthode `S256` est requis pour chaque client.

[inline-code-attrs-start title = 'Requête d\'autorisation'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Paramètres de la requête d\'autorisation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

L'utilisateur se connecte à FastComments si nécessaire et voit une page de consentement indiquant votre application, le compte auquel elle sera connectée, et les portées demandées. L'utilisateur doit posséder la permission **API Admin** sur ce compte ; toute autre personne voit une erreur de permission au lieu du formulaire de consentement. L'approbation redirige le navigateur vers votre `redirect_uri` avec `code` et `state`. Le refus redirige avec `error=access_denied`.

Le code d'autorisation est valable pendant 10 minutes et ne peut être échangé qu'une fois. Un second échange du même code révoque tous les jetons produits par le premier échange.

### Étape 2 - Requête de jeton

Échangez le code contre des jetons. Le corps est encodé en formulaire. Les clients confidentiels s'authentifient avec `client_secret_basic` (HTTP Basic) ou `client_secret_post` (secret dans le corps). Les clients publics envoient uniquement `client_id`.

[inline-code-attrs-start title = 'Exemple cURL de requête de jeton'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Corps de la requête de jeton (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structure de la réponse de jeton'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Les erreurs suivent la RFC 6749 : un corps JSON avec `error` et `error_description`, HTTP 400 pour `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` et `unsupported_grant_type`, HTTP 401 pour `invalid_client`, HTTP 429 en cas de limitation de débit.

### Étape 3 - Appel de l'API

Envoyez le jeton d'accès comme jeton porteur. Le locataire est implicite dans le jeton, donc `tenantId` est optionnel. S'il est fourni, il doit correspondre au jeton sinon la requête échoue.

[inline-code-attrs-start title = 'Exemple cURL de jeton porteur'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` renvoie le locataire, l'utilisateur autorisant, et les portées accordées, ce qui en fait l'appel approprié pour un test de connexion. Une requête avec un jeton expiré ou révoqué reçoit HTTP 401. Une requête dont la méthode nécessite une portée que le jeton ne possède pas reçoit HTTP 403.

### Étape 4 - Rafraîchissement

[inline-code-attrs-start title = 'Exemple cURL de requête de rafraîchissement'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Corps de la requête de jeton (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

La réponse a la même forme que l'échange de code. Les jetons de rafraîchissement tournent : chaque rafraîchissement renvoie un nouveau `refresh_token` et révoque l'ancien après une fenêtre de grâce de 30 secondes pour les requêtes simultanées. Présenter un jeton de rafraîchissement qui a été tourné il y a plus de 30 secondes est considéré comme une relecture et révoque l'ensemble de l'octroi. Les applications partenaires enregistrées par FastComments sont exemptées de rotation et récupèrent le même jeton de rafraîchissement avec son expiration prolongée de 30 jours supplémentaires.

Un rafraîchissement vérifie également que l'utilisateur autorisant possède toujours le rôle API Admin sur le compte. Sinon, l'octroi est révoqué et la réponse est `invalid_grant`.

### Révocation

[inline-code-attrs-start title = 'Exemple cURL de requête de révocation'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Révoquer un jeton de rafraîchissement révoque chaque jeton d'accès émis à partir du même octroi. Révoquer un jeton d'accès ne révoque que ce jeton. Le point de terminaison renvoie HTTP 200 avec un objet JSON vide que le jeton soit trouvé ou non, selon la RFC 7009.

Les utilisateurs peuvent également révoquer une connexion depuis **Connected Apps** dans le tableau de bord FastComments. Tous les jetons pour cette application cessent de fonctionner immédiatement.

---