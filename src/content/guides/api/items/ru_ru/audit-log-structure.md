An `AuditLog` — это объект, представляющий проверяемое событие для арендаторов, имеющих доступ к этой функции.

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

`targetId` и `targetLabel` описывают, над чем было выполнено событие; `userId` и `username` описывают, кто его выполнил. Для обновлений `objectDetails.changes` содержит карту `{field: {from, to}}`, показывающую, что именно изменилось.

Журнал аудита является неизменяемым. Его также нельзя записывать вручную. FastComments.com может решать, когда записывать в журнал аудита. Тем не менее, вы можете читать его через этот API.

События в журнале аудита истекают через два года.