Um objeto `Subscription` representa uma assinatura de um usuário.

Objetos `Subscription` são criados quando um usuário clica no sininho de notificações no widget de comentários e clica em "Subscribe to this page".

Assinaturas também podem ser criadas via API.

Ter um objeto `Subscription` faz com que objetos `Notification` sejam gerados e e-mails sejam enviados quando novos comentários são deixados na raiz da página associada
para a qual a `Subscription` foi criada. O envio de e-mails depende do tipo de usuário. Para usuários regulares, isso depende de `optedInNotifications`. Para usuários SSO, depende de `optedInSubscriptionNotifications`. Observe que alguns aplicativos podem não ter o conceito de uma página acessível pela web, caso em que basta definir `urlId` como
o id do item ao qual você está assinando (o mesmo valor de `urlId` que você passaria para o widget de comentários).

A estrutura do objeto `Subscription` é a seguinte:

[inline-code-attrs-start title = 'Estrutura de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Com SSO, o id do usuário está no formato `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]