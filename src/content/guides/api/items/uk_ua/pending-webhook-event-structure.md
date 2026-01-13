Об'єкт `PendingWebhookEvent` представляє подію вебхука, яка знаходиться в черзі та очікує виконання.

`PendingWebhookEvent` об'єкти створюються автоматично і не можуть бути створені вручну через API. Їх термін дії також закінчується через один рік.
Їх можна видалити, що видаляє завдання з черги.

Існують різні типи подій — перевіряйте `eventType` (`OutboundSyncEventType`) і `type` (`OutboundSyncType`).

Поширений випадок використання цього API — реалізація власного моніторингу. Можливо, ви захочете періодично викликати ендпоінт `/count`
щоб опитувати кількість невиконаних завдань за заданими фільтрами.

Структура об'єкта `PendingWebhookEvent` має такий вигляд:

[inline-code-attrs-start title = 'Структура PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Синхронізація, специфічна для WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** Ідентифікатор коментаря, пов'язаний із подією. **/
    commentId: string
    /** Об'єкт коментаря для події на момент її виникнення. Ми почали додавати їх у листопаді 2023 року. **/
    comment: Comment
    /** Зовнішній ідентифікатор, який може бути пов'язаний із коментарем. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Встановлюється перед першою спробою та після кожної невдачі. **/
    nextAttemptAt: Date
    /** Чи є це подія створення, видалення або оновлення... **/
    eventType: OutboundSyncEventType
    /** Тип синхронізації для виконання (WordPress, виклик API тощо). **/
    type: OutboundSyncType
    /** Домен, який відповідав коментарю. Ми використовуємо цей домен для вибору API-ключа. **/
    domain: string
    /** Остання помилка, яка сталася. Цей тип не має строгої типізації і є "дампом" того, що трапилось. Зазвичай містить об'єкт зі статусним кодом, тілом і мапою заголовків. **/
    lastError: object | null
}
[inline-code-end]

---