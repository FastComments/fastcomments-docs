[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO usa criptografia HMAC-SHA256 como o mecanismo para implementar SSO. Primeiro iremos explicar a arquitetura geral, fornecer exemplos e passos detalhados.

Também há documentação sobre migração de outros provedores com mecanismos de SSO semelhantes, e as diferenças.

O fluxo é o seguinte:

<div class="screenshot white-bg">
    <div class="title">Fluxo SSO Seguro</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagrama SSO Seguro" />
</div>

Como o Secure SSO envolve desenvolvimento full-stack, exemplos de código completos e funcionais em Java/Spring, NodeJS/Express e PHP vanilla estão atualmente <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">no GitHub</a>.

Embora usemos ExpressJS no exemplo NodeJS e Spring no exemplo Java, não são necessárias bibliotecas/frames de trabalho nesses runtimes para implementar o FastComments SSO - os pacotes nativos de criptografia funcionam.

Você não precisa escrever novos endpoints da API com o FastComments SSO. Simplesmente criptografe as informações do usuário usando sua chave secreta e passe o payload para o widget de comentários.

#### Obtenha sua chave secreta da API

Sua chave secreta da API pode ser recuperada a partir desta <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">página</a>. Você também pode encontrar esta página indo para My Account, clicando no bloco API/SSO e então clicando em "Get API Secret Key".

#### Parâmetros do widget de comentários

A documentação de alto nível da API para o widget de comentários pode ser encontrada <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">aqui</a>.

Vamos aprofundar o que esses parâmetros significam.

O widget de comentários recebe um objeto de configuração - você já passa isso se estiver usando o FastComments para enviar seu id de cliente (chamado tenantId).

Para habilitar SSO, passe um novo objeto "sso", que deve ter os seguintes parâmetros. Os valores devem ser gerados no servidor.

- userDataJSONBase64: Os dados do usuário em formato JSON, que então são codificados em Base64.
- verificationHash: O hash HMAC-SHA256 criado a partir de UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Timestamp em epoch, em **milissegundos**. Não deve estar no futuro, nem com mais de dois dias no passado.
- loginURL: Uma URL que o widget de comentários pode mostrar para autenticar o usuário.
- logoutURL: Uma URL que o widget de comentários pode mostrar para desconectar o usuário.
- loginCallback: Quando fornecido em vez do login URL, uma função que o widget de comentários irá invocar ao clicar no botão de login.
- logoutCallback: Quando fornecido em vez do logout URL, uma função que o widget de comentários irá invocar ao clicar no botão de logout.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### O Objeto do Usuário

The User Object
[inline-code-attrs-start title = 'O Objeto do Usuário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obrigatório. Máximo de 1k caracteres. **/
    id: string;
    /** Obrigatório. Máximo de 1k caracteres. Observação: Deve ser único. **/
    email: string;
    /** Obrigatório. Máximo de 1k caracteres. Observação: O nome de usuário não pode ser um e-mail. Não precisa ser único. **/
    username: string;
    /** Opcional. Máximo de 3k caracteres para URLs. Padrão é do gravatar baseado no email. Suporta imagens codificadas em base64, nesse caso o limite é de 50k caracteres. **/ 
    avatar?: string;
    /** Opcional. Padrão: false. **/
    optedInNotifications?: boolean;
    /** Opcional. Padrão: false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcional. Máximo de 100 caracteres. Este rótulo será mostrado ao lado do nome. Padrão é Administrator/Moderator quando aplicável. **/
    displayLabel?: string;
    /** Opcional. Máximo de 500 caracteres. Isso será exibido em vez do username. **/
    displayName?: string;
    /** Opcional. Máximo de 2k caracteres. O nome do usuário será vinculado a este. **/
    websiteUrl?: string;
    /** Opcional. Até 100 grupos por usuário. Um id de grupo não pode ter mais que 50 caracteres. **/
    groupIds?: string[];
    /** Opcional. Indica que o usuário é um administrador. **/
    isAdmin?: boolean;
    /** Opcional. Indica que o usuário é um moderador. **/
    isModerator?: boolean;
    /** Opcional, padrão true. Configure como false para habilitar a aba "activity" no perfil do usuário. **/
    isProfileActivityPrivate?: boolean;
    /** Opcional, padrão false. Configure como true para desabilitar comentários no perfil. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcional, padrão false. Configure como true para desabilitar mensagens diretas para este usuário. **/
    isProfileDMDisabled?: boolean;
    /** Configuração opcional para badges do usuário. **/
    badgeConfig?: {
        /** Array de IDs de badges globais a atribuir. Limitado a 30 badges. A ordem é respeitada. **/
        badgeIds: string[];
        /** Array de IDs de badges com escopo para a página atual (urlId). Exibido somente na página atribuída. **/
        pageBadgeIds?: string[];
        /** Se true, substitui badges exibidos existentes. Global e com escopo de página são sobrescritos independentemente. **/
        override?: boolean;
        /** Se true, atualiza propriedades de exibição das badges a partir da configuração do tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderadores e Administradores

Para admins e moderadores, passe as respectivas flags `isAdmin` ou `isModerator` no objeto `SSOUser`.

#### Notificações

Para habilitar ou desabilitar notificações, defina o valor de `optedInNotifications` como `true` ou `false`, respectivamente. Na primeira vez que o usuário carregar a página com esse valor no payload SSO, as configurações de notificação dele serão atualizadas.

Além disso, se você quiser que os usuários recebam e-mails de notificação por atividade em páginas às quais se inscreveram (em oposição a apenas notificações no aplicativo), então defina `optedInSubscriptionNotifications` como `true`.

#### Usuários VIP & Rótulos Especiais

Você pode exibir um rótulo especial ao lado do nome do usuário usando o campo opcional "displayLabel".

#### Usuários não autenticados

Para representar um usuário não autenticado, simplesmente não preencha userDataJSONBase64, verificationHash ou timestamp. Forneça um loginURL.

Esses usuários não poderão comentar e, em vez disso, serão apresentados com uma mensagem de login (mensagem, link ou botão, dependendo da configuração).

#### Exemplos diretos para serializar e gerar hash dos dados do usuário

Mais detalhes e exemplos <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">aqui</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">aqui</a> (java) e <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">aqui</a> (php).

Entendemos que qualquer integração pode ser um processo complicado e doloroso. Não hesite em contatar seu representante ou usar a <a href="https://fastcomments.com/auth/my-account/help" target="_blank">página de suporte</a>.

---