Um objeto `Notification` representa uma notificação para um usuário.

Objetos `Notification` são criados automaticamente e não podem ser criados via API. Eles também expiram após um ano.
Notificações não podem ser excluídas. Elas podem, no entanto, ser atualizadas para definir `viewed` como `false`, e você pode consultar por `viewed`.

Um usuário também pode optar por não receber notificações para um comentário específico configurando `optedOut` na notificação para `true`. Você pode optar por receber novamente definindo para `false`.

Existem diferentes tipos de notificação - verifique `relatedObjectType` e `type`.

As formas como notificações são criadas são bastante flexíveis e podem ser acionadas por muitos cenários (veja `NotificationType`).

Atualmente, a existência de uma `Notification` não implica necessariamente que um e-mail foi ou deve ser enviado. Em vez disso, as notificações
são usadas para o feed de notificações e integrações relacionadas.

A estrutura do objeto `Notification` é a seguinte:

[inline-code-attrs-start title = 'Estrutura de Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Se alguém respondeu para você. **/
    RepliedToMe = 0,
    /** Se alguém respondeu em qualquer lugar de uma thread (até filhos de filhos) de uma thread na qual você comentou. **/
    RepliedTransientChild = 1,
    /** Se seu comentário foi votado positivamente. **/
    VotedMyComment = 2,
    /** Se um novo comentário for deixado na raiz de uma página à qual você está inscrito. **/
    SubscriptionReplyRoot = 3,
    /** Se alguém comentou no seu perfil. **/
    CommentedOnProfile = 4,
    /** Se você tiver uma mensagem direta (DM). **/
    DirectMessage = 5,
    /** TrialLimits é apenas para usuários do tenant. **/
    TrialLimits = 6,
    /** Se você foi mencionado com @. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId?: string
    /** Ao trabalhar com SSO, você só precisa se preocupar com `userId`. **/
    anonUserId?: string
    /** urlId quase sempre está definido. Ele é opcional apenas para notificações em nível de tenant, que são infrequentes. **/
    urlId?: string
    /** A URL é armazenada em cache para navegação rápida até a origem da notificação. **/
    url?: string
    /** O título da página é armazenado em cache para leitura rápida da origem da notificação. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Por exemplo, id do comentário. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // string de data
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName e fromUserAvatarSrc são armazenados em cache aqui para exibição rápida da notificação. Eles são atualizados quando o usuário é atualizado. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Defina isto como true para parar de receber notificações para este objeto. **/
    optedOut?: boolean
}
[inline-code-end]

---