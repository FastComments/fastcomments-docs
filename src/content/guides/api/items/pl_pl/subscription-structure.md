Obiekt `Subscription` reprezentuje subskrypcję dla użytkownika.

Obiekty `Subscription` są tworzone, gdy użytkownik kliknie dzwonek powiadomień w widżecie komentarzy i wybierze "Subskrybuj tę stronę".

Subskrypcje można również tworzyć za pomocą API.

Posiadanie obiektu `Subscription` powoduje wygenerowanie obiektów `Notification` i wysłanie e-maili, gdy nowe komentarze zostaną umieszczone w korzeniu powiązanej strony
na którą dotyczy `Subscription`. Wysyłanie e-maili zależy od typu użytkownika. Dla zwykłych użytkowników zależy to od `optedInNotifications`. Dla użytkowników SSO zależy to od `optedInSubscriptionNotifications`. Zwróć uwagę, że niektóre aplikacje mogą nie mieć pojęcia strony dostępnej w sieci, w takim przypadku po prostu ustaw `urlId` na
identyfikator elementu, który subskrybujesz (ta sama wartość `urlId`, którą przekazałbyś do widżetu komentarzy).

Struktura obiektu `Subscription` jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** W przypadku SSO identyfikator użytkownika ma format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // łańcuch daty
}
[inline-code-end]

---