Bir `AuditLog`, bu özelliğe erişimi olan kiracılar (tenant) için denetlenmiş bir olayı temsil eden bir nesnedir.

AuditLog nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'AuditLog Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Olayı gerçekleştiren kişi. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Olayı gerçekleştiren tarayıcı, olay bir tarayıcıdan geldiyse. **/
    ua?: string;
    /** Olayın geldiği oturumun hash'i, bir kişinin eylemlerini ilişkilendirmek için. Asla oturumun kendisi değildir. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** Olayın gerçekleştirildiği nesnenin kimliği, olayı gerçekleştiren kişinin aksine. **/
    targetId?: string;
    /** Bu nesne için insan tarafından okunabilir bir etiket, örn. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` ve `targetLabel`, olayın ne üzerinde gerçekleştirildiğini açıklar; `userId` ve `username` ise onu kimin gerçekleştirdiğini açıklar. Güncellemeler için `objectDetails.changes`, gerçekte neyin değiştiğini gösteren bir `{field: {from, to}}` haritası içerir.

Denetim günlüğü değiştirilemezdir. Ayrıca manuel olarak yazılamaz. FastComments.com, denetim günlüğüne ne zaman yazılacağına yalnızca kendisi karar verebilir. Ancak bu API aracılığıyla ondan okuyabilirsiniz.

Denetim günlüğündeki olaylar iki yıl sonra sona erer.