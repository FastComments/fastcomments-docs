Obiekt `PendingWebhookEvent` reprezentuje zdarzenie webhooka w kolejce oczekujące na przetworzenie.

Obiekty `PendingWebhookEvent` są tworzone automatycznie i nie można ich tworzyć ręcznie przez API. Wygasają też po roku.
Można je usunąć, co usuwa zadanie z kolejki.

Istnieją różne typy zdarzeń - sprawdź `eventType` (`OutboundSyncEventType`) oraz `type` (`OutboundSyncType`).

Typowym przypadkiem użycia tego API jest implementacja niestandardowego monitorowania. Możesz chcieć okresowo wywoływać endpoint `/count`
aby odpytywać liczbę oczekujących zadań dla określonych filtrów.

Struktura obiektu `PendingWebhookEvent` jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Zadanie synchronizacji specyficzne dla WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** Id komentarza powiązany ze zdarzeniem. **/
    commentId: string
    /** Obiekt komentarza dla zdarzenia w chwili jego wystąpienia. Zaczęliśmy dodawać je w listopadzie 2023. **/
    comment: Comment
    /** Zewnętrzne id, które może być powiązane z komentarzem. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Ustawiane przed pierwszą próbą i po każdym niepowodzeniu. **/
    nextAttemptAt: Date
    /** Czy jest to zdarzenie utworzenia, usunięcia czy aktualizacji... **/
    eventType: OutboundSyncEventType
    /** Typ synchronizacji do wykonania (WordPress, wywołanie API itp.). **/
    type: OutboundSyncType
    /** Domena, która dopasowała komentarz. Używamy tej domeny do wyboru klucza API. **/
    domain: string
    /** Ostatni występujący błąd. Ten typ jest nietypowany i jest „zrzutem” tego, co się stało. Zazwyczaj zawiera obiekt ze statusCode, body i mapą headers. **/
    lastError: object | null
}
[inline-code-end]

---