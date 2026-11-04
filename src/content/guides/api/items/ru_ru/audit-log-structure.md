`AuditLog` — это объект, представляющий аудируемое событие для клиентов, у которых есть доступ к этой функции.

Структура объекта AuditLog выглядит следующим образом:

[inline-code-attrs-start title = 'Структура AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Кто выполнил событие. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Браузер, из которого было выполнено событие, если оно пришло из браузера. **/
    ua?: string;
    /** Хэш сессии, из которой пришло событие, для соотнесения действий одного человека. Никогда сама сессия. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** Идентификатор объекта, над которым было выполнено событие, в отличие от того, кто его выполнил. **/
    targetId?: string;
    /** Понятная человеку метка этого объекта, например "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` и `targetLabel` описывают, над чем было выполнено событие; `userId` и `username` описывают, кто его выполнил. Для обновлений `objectDetails.changes` содержит карту `{field: {from, to}}` того, что фактически изменилось.

Журнал аудита неизменяем. В него также нельзя писать вручную. Только FastComments.com решает, когда записывать данные в журнал аудита. Однако вы можете читать из него через этот API.

События в журнале аудита удаляются через два года.