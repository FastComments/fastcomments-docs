Um `Subscription` object representa uma assinatura para um usuário.

`Subscription` objects are created when a user clicks the notification bell in the comment widget and clicks "Inscrever-se nesta página".

Subscriptions can also be created via the API.

Ter um objeto `Subscription` faz com que `Notification` objects sejam gerados, e emails enviados, quando novos comentários são deixados na raiz da página associada
à qual a `Subscription` se refere. O envio de emails depende do tipo de usuário. Para usuários comuns isso depende de `optedInNotifications`. Para usuários SSO isso depende de `optedInSubscriptionNotifications`. Observe que algumas aplicações podem não ter o conceito de uma página acessível pela web, caso em que basta definir `urlId` como
o id do item ao qual você está se inscrevendo (mesmo valor para `urlId` que você passaria para o widget de comentários).

A estrutura para o `Subscription` object é a seguinte:

[inline-code-attrs-start title = 'Estrutura de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Com SSO, o id do usuário tem o formato `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // string de data
}
[inline-code-end]