Ein `AuditLog` ist ein Objekt, das ein geprüftes Ereignis für Mandanten darstellt, die Zugriff auf diese Funktion haben.

Die Struktur des AuditLog-Objekts ist wie folgt:

[inline-code-attrs-start title = 'AuditLog Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

`targetId` und `targetLabel` beschreiben, worauf das Ereignis angewendet wurde; `userId` und `username` beschreiben, wer es ausgeführt hat. Bei Aktualisierungen enthält `objectDetails.changes` eine `{field: {from, to}}`-Karte dessen, was tatsächlich geändert wurde.

Das Audit-Log ist unveränderlich. Es kann auch nicht manuell geschrieben werden. FastComments.com kann nur entscheiden, wann in das Audit-Log geschrieben wird. Sie können jedoch über diese API darauf zugreifen.

Ereignisse im Audit-Log verfallen nach zwei Jahren.