A `Subscription` објекат представља претплату за корисника.

`Subscription` објекти се креирају када корисник кликне на звоник за обавештења у виџету за коментаре и кликне „Претплати се на ову страницу“.

Претплате се такође могу креирати преко API‑ја.

Постојање `Subscription` објекта доводи до генерисања `Notification` објеката и слања имејлова када се нови коментари оставе на корену повезане странице за коју је `Subscription` направљена. Слање имејлова зависи од типа корисника. За обичне кориснике ово зависи од `optedInNotifications`. За SSO кориснике ово зависи од `optedInSubscriptionNotifications`. Имајте на уму да неке апликације можда неће имати концепт веб‑приступачне странице, у ком случају једноставно поставите `urlId` на
ид ставке којој се претплаћујете (исту вредност за `urlId` коју бисте проследили виџету за коментаре).

Структура за `Subscription` објекат је следећа:

[inline-code-attrs-start title = 'Struktura pretplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Sa SSO, ID korisnika je u formatu `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]