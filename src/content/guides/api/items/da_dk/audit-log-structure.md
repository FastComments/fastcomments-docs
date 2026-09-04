Et `AuditLog` er et objekt, der repræsenterer en revideret hændelse for lejere, der har adgang til denne funktion.

Strukturen for AuditLog-objektet er som følger:

[inline-code-attrs-start title = 'AuditLog Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Hvem udførte hændelsen. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Browseren der udførte hændelsen, når den kom fra en. **/
    ua?: string;
    /** Et hash af sessionen, som hændelsen kom fra, for at korrelere en persons handlinger. Aldrig selve sessionen. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** Id'et på objektet, som hændelsen blev udført på, i modsætning til hvem der udførte den. **/
    targetId?: string;
    /** En menneskelig læsbar etiket for det objekt, f.eks. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` og `targetLabel` beskriver, hvad hændelsen blev udført på; `userId` og `username` beskriver, hvem der udførte den. For opdateringer indeholder `objectDetails.changes` et `{field: {from, to}}` kort over, hvad der faktisk ændrede sig.

Auditloggen er uforanderlig. Den kan heller ikke skrives til manuelt. FastComments.com kan kun beslutte, hvornår der skrives til auditloggen. Du kan dog læse fra den via dette API.

Hændelser i auditloggen udløber efter to år.