Bir `AuditLog`, bu özelliğe erişimi olan kiracılar için denetlenmiş bir olayı temsil eden bir nesnedir.

AuditLog nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'AuditLog Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    objectDetails?: object;
}
[inline-code-end]

Denetim günlüğü değiştirilemez. Ayrıca elle yazılamaz. FastComments.com yalnızca denetim günlüğüne ne zaman yazılacağına karar verebilir. Ancak bu API aracılığıyla ondan okuyabilirsiniz.

Denetim günlüğündeki olayların geçerliliği iki yıl sonra sona erer.