Објекат `Subscription` представља претплату за корисника.

`Subscription` објекти се креирају када корисник кликне на звоно обавештења у видгету за коментаре и изабере "Претплати се на ову страницу".

Претплате се такође могу креирати путем API-ја.

Поседовање објекта `Subscription` узрокује да се генеришу `Notification` објекти и да се шаљу мејлови када се оставе нови коментари на корену повезане странице за коју та `Subscription` важи. Слање мејлова зависи од типа корисника. За обичне кориснике то зависи од `optedInNotifications`. За SSO кориснике то зависи од `optedInSubscriptionNotifications`. Имајте у виду да неке апликације можда немају концепт веб-приступачне странице, у ком случају једноставно поставите `urlId` на id ставке на коју се претплаћујете (иста вредност за `urlId` коју бисте проследили видгету за коментаре).

Структура објекта `Subscription` је следећа:

[inline-code-attrs-start title = 'Структура Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** За SSO, кориснички id је у формату `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // стринг датума
}
[inline-code-end]

---