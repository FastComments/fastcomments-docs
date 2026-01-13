An `AuditLog` je objekt, ki predstavlja revidiran dogodek za najemnike, ki imajo dostop do te funkcije.

Struktura objekta AuditLog je naslednja:

[inline-code-attrs-start title = 'Struktura objekta AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Revizijski dnevnik je nespremenljiv. Vanj poleg tega ni mogoče pisati ročno. FastComments.com lahko odloča le, kdaj zapisati v revizijski dnevnik. Vendar ga lahko berete prek tega API-ja.

Dogodki v revizijskem dnevniku potečejo po dveh letih.

---