An `AuditLog` je objekt, ki predstavlja revizijski dogodek za najemnike, ki imajo dostop do te funkcije.

Struktura objekta AuditLog je naslednja:

[inline-code-attrs-start title = 'Struktura AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Kdo je izvedel dogodek. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Brskalnik, ki je izvedel dogodek, ko je prišel iz enega. **/
    ua?: string;
    /** Zgoščena vrednost seje, iz katere je prišel dogodek, za povezovanje dejanj ene osebe. Nikoli sama seja. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** ID objekta, na katerem je bil dogodek izveden, v nasprotju s tem, kdo ga je izvedel. **/
    targetId?: string;
    /** Človeško berljiva oznaka za ta objekt, npr. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` in `targetLabel` opisujeta, na čem je bil dogodek izveden; `userId` in `username` opisujeta, kdo ga je izvedel. Za posodobitve `objectDetails.changes` vsebuje `{field: {from, to}}` zemljevid tega, kaj se je dejansko spremenilo.

Revizijski dnevnik je nepreminljiv. Prav tako ga ni mogoče ročno zapisati. FastComments.com lahko sam odloči, kdaj zapisati v revizijski dnevnik. Vendar ga lahko preberete prek tega API-ja.

Dogodki v revizijskem dnevniku potečejo po dveh letih.