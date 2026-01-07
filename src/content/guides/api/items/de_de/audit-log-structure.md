Ein `AuditLog` ist ein Objekt, das ein auditiertes Ereignis für Tenants darstellt, die Zugriff auf diese Funktion haben.

Die Struktur für das AuditLog-Objekt ist wie folgt:

[inline-code-attrs-start title = 'AuditLog Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Das Audit-Log ist unveränderlich. Es kann auch nicht manuell beschrieben werden. Nur FastComments.com entscheidet, wann ins Audit-Log geschrieben wird. Sie können es jedoch über diese API lesen.

Ereignisse im Audit-Log verfallen nach zwei Jahren.
