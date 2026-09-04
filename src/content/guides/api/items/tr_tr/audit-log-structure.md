An `AuditLog` bir nesnedir ve bu özelliğe erişimi olan kiracılar için denetlenen bir olayı temsil eder.

The structure for the AuditLog object is as follows:

[inline-code-attrs-start title = 'AuditLog Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

`targetId` ve `targetLabel` olayın ne üzerinde gerçekleştirildiğini tanımlar; `userId` ve `username` ise kim tarafından gerçekleştirildiğini tanımlar. Güncellemeler için, `objectDetails.changes` gerçekte neyin değiştiğini gösteren `{field: {from, to}}` haritasını tutar.

Denetim günlüğü değiştirilemezdir. Ayrıca manuel olarak yazılamaz. FastComments.com yalnızca denetim günlüğüne ne zaman yazılacağına karar verebilir. Bununla birlikte, bu API aracılığıyla denetim günlüğünü okuyabilirsiniz.

Denetim günlüğündeki olaylar iki yıl sonra süresi dolar.