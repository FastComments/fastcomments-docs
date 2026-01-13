`AuditLog` je objekat koji predstavlja evidentirani događaj za tenant-e koji imaju pristup ovoj funkciji.

Struktura za `AuditLog` objekat je sledeća:

[inline-code-attrs-start title = 'Struktura AuditLog objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Audit log je nepromenljiv. Takođe se u njega ne može ručno pisati. FastComments.com jedino odlučuje kada će se upisati u audit log. Međutim, možete ga čitati putem ovog API-ja.

Događaji u audit logu ističu nakon dve godine.