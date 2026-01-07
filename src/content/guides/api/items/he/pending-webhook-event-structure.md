אובייקט `PendingWebhookEvent` מייצג אירוע webhook בתור שממתין.

אובייקטי `PendingWebhookEvent` נוצרים אוטומטית ולא ניתן ליצור אותם ידנית דרך ה-API. הם גם פגים לאחר שנה אחת.
ניתן למחוק אותם מה שמסיר את המשימה מהתור.

יש סוגי אירועים שונים - בדוק `eventType` (`OutboundSyncEventType`) ו-`type` (`OutboundSyncType`).

מקרה שימוש נפוץ ל-API זה הוא ליישם ניטור מותאם אישית. ייתכן שתרצה לקרוא לנקודת הקצה `/count` מעת לעת
כדי לבדוק את הספירה הממתינה עבור מסננים נתונים.

המבנה עבור אובייקט `PendingWebhookEvent` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress-specific sync task. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** The comment id associated with the event. **/
    commentId: string
    /** The comment object for the event at the time of the event. We started adding these in Nov 2023. **/
    comment: Comment
    /** An external id that may be associated with the comment. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Set before first attempt, and after every failure. **/
    nextAttemptAt: Date
    /** Whether this is a creation, deletion, or update event... **/
    eventType: OutboundSyncEventType
    /** The type of sync to perform (WordPress, call API, etc). **/
    type: OutboundSyncType
    /** The domain that matched the comment. We use this domain to pick the API key. **/
    domain: string
    /** The last error that occurred. This type is untyped and is a "dump" of whatever happened. Usually it contains an object with statusCode, body, and a headers map. **/
    lastError: object | null
}
[inline-code-end]
