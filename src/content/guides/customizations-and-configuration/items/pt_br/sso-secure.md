[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO usa criptografia HMAC-SHA256 como mecanismo para implementar SSO. Primeiro vamos abordar a arquitetura geral, fornecer exemplos e passos detalhados.

There is also some documentation regarding migrating from other providers with similar SSO mechanisms, and the differences.

The flow looks like this:

<div class="screenshot white-bg">
    <div class="title">Fluxo de SSO Seguro</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagrama SSO Seguro" />
</div>

Since Secure SSO involves full-stack development, full working code examples in Java/Spring, NodeJS/Express, and vanilla PHP are currently <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">on GitHub</a>.

Although we use ExpressJS in the NodeJS example and Spring in the Java example there are no frameworks/libraries required in these run-times to implement FastComments SSO - the native crypto packages work.

You don't have to write any new API endpoints with FastComments SSO. Simply encrypt the user's info using your secret key and pass the payload to the comment widget.

#### Obtenha sua Chave Secreta da API

Your API Secret can be retrieved from <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">this page</a>. You can find this page also by going to My Account, clicking the API/SSO tile, and then clicking "Get API Secret Key".

#### Parâmetros do Widget de Comentários

High-level API documentation for the comment widget can be found <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">here</a>.

Let's go into more detail of what these parameters mean.

The comment widget takes a configuration object - you already pass this if you're using FastComments to pass your customer id (called tenantId).

To enable SSO, pass a new "sso" object, which must have the following parameters. The values should be generated server side.

- userDataJSONBase64: The user's data in JSON format, which is then Base64 encoded.
- verificationHash: The HMAC-SHA256 hash created from UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, in **milliseconds**. Must not be in the future, or more than two days in the past.
- loginURL: A URL that the comment widget can show to log the user in.
- logoutURL: A URL that the comment widget can show to log the user out.
- loginCallback: When provided instead of the login URL, a function that the comment widget will invoke when clicking the login button.
- logoutCallback: When provided instead of the logout URL, a function that the comment widget will invoke when clicking the logout button.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### O objeto do usuário

The User object contains the following schema:
[inline-code-attrs-start title = 'O objeto do usuário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obrigatório. Máx. 1k caracteres. **/
    id: string;
    /** Obrigatório. Máx. 1k caracteres. Observação: Deve ser único. **/
    email: string;
    /** Obrigatório. Máx. 1k caracteres. Observação: O nome de usuário não pode ser um email. Não precisa ser único. **/
    username: string;
    /** Opcional. Máx. 3k caracteres para URLs. O padrão vem do gravatar baseado no email. Suporta imagens codificadas em base64, caso em que o limite é 50k caracteres. **/ 
    avatar?: string;
    /** Opcional. Padrão: false. **/
    optedInNotifications?: boolean;
    /** Opcional. Padrão: false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcional. Máx. 100 caracteres. Este rótulo será mostrado ao lado do nome. Padrão: Administrador/Moderador quando aplicável. **/
    displayLabel?: string;
    /** Opcional. Máx. 500 caracteres. Isso será mostrado em vez do nome de usuário. **/
    displayName?: string;
    /** Opcional. Máx. 2k caracteres. O nome do usuário será vinculado a este URL. **/
    websiteUrl?: string;
    /** Opcional. Até 100 grupos por usuário. Um id de grupo não pode ter mais de 50 caracteres. **/
    groupIds?: string[];
    /** Opcional. Indica o usuário como administrador. **/
    isAdmin?: boolean;
    /** Opcional. Indica o usuário como moderador. **/
    isModerator?: boolean;
    /** Opcional, padrão true. Defina como false para habilitar a aba "activity" no perfil do usuário. **/
    isProfileActivityPrivate?: boolean;
    /** Opcional, padrão false. Defina como true para desativar comentários no perfil. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcional, padrão false. Defina como true para desativar mensagens diretas para este usuário. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

For admins and moderators, pass the respective `isAdmin` or `isModerator` flags in the `SSOUser` object.

#### Notificações

To enable or disable notifications, set the value of `optedInNotifications` to `true` or `false` respectively. The first time the user loads the page with this value in the SSO payload, their notification settings will be updated.

Additionally, if you want users to receive notification emails for activity on pages they subscribed to (as opposed to just in-app notifications), then set `optedInSubscriptionNotifications` to `true`.

#### Usuários VIP e Rótulos Especiais

You can display a special label next to the user's name by using the optional "displayLabel" field.

#### Usuários não autenticados

To represent an unauthenticated user, simply do not populate userDataJSONBase64, verificationHash, or timestamp. Provide a loginURL.

These users will not be able to comment, and instead will be presented with a login message (message, link, or button, depending on configuration).

#### Exemplos diretos para Serializar e Gerar Hash dos Dados do Usuário

More details as an examples <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">here</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">here</a> (java) and <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">here</a> (php).

We understand that any integration can be a complicated and painful process. Don't hesitate to reach out to your representative or use the <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support page</a>.

---