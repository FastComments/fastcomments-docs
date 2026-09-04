An `AuditLog` is an object that represents an audited event for tenants that have access to this feature.

The structure for the AuditLog object is as follows:

[inline-code-attrs-start title = 'Структура AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Who performed the event. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** The browser that performed the event, when it came from one. **/
    ua?: string;
    /** A hash of the session the event came from, for correlating one person's actions. Never the session itself. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** The id of the object the event was performed on, as opposed to who performed it. **/
    targetId?: string;
    /** A human-readable label for that object, e.g. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` і `targetLabel` описують, на що була виконана подія; `userId` і `username` описують, хто її виконав. Для оновлень `objectDetails.changes` містить карту `{field: {from, to}}`, що показує, що саме змінилося.

Журнал аудиту є незмінним. Його також не можна записувати вручну. FastComments.com може вирішувати, коли записувати в журнал аудиту. Однак ви можете читати його за допомогою цього API.

Події в журналі аудиту видаляються через два роки.