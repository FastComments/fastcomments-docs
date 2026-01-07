`AuditLog` je objekt koji predstavlja revidirani događaj za stanare koji imaju pristup ovoj značajci.

Struktura za AuditLog objekt je sljedeća:

[inline-code-attrs-start title = 'Struktura AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Revizijski zapisnik je nepromjenjiv. Također se ne može ručno pisati u njega. FastComments.com može odlučiti samo kada pisati u revizijski zapisnik. Međutim, možete ga čitati putem ovog API-ja.

Događaji u revizijskom zapisniku istječu nakon dvije godine.
