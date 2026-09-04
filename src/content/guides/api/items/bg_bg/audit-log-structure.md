An `AuditLog` е обект, който представлява одитирано събитие за наематели, които имат достъп до тази функция.

Структурата на обекта AuditLog е следната:

[inline-code-attrs-start title = 'Структура на AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

`targetId` и `targetLabel` описват върху какво е извършено събитието; `userId` и `username` описват кой го е извършил. При актуализации, `objectDetails.changes` съдържа карта `{field: {from, to}}` на това, което действително е променено.

Одитният журнал е неизменяем. Също така не може да се записва ръчно. FastComments.com може единствено да реши кога да записва в одитния журнал. Въпреки това, можете да четете от него чрез това API.

Събитията в одитния журнал изтичат след две години.