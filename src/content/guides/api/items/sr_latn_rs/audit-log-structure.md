An `AuditLog` je objekat koji predstavlja revizijski događaj za zakupce koji imaju pristup ovoj funkciji.

Struktura za objekat AuditLog je sledeća:

[inline-code-attrs-start title = 'Struktura AuditLog-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Ko je izvršio događaj. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Pregledač koji je izvršio događaj, kada je došao iz pregledača. **/
    ua?: string;
    /** Hash sesije iz koje je događaj došao, za povezivanje radnji jedne osobe. Nikada sama sesija. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** ID objekta na kojem je događaj izvršen, nasuprot tome ko ga je izvršio. **/
    targetId?: string;
    /** Ljudski čitljiva oznaka za taj objekat, npr. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` i `targetLabel` opisuju na čemu je događaj izvršen; `userId` i `username` opisuju ko ga je izvršio. Za ažuriranja, `objectDetails.changes` sadrži `{field: {from, to}}` mapu onoga što se zaista promenilo.

Revizijski zapis je nepromenljiv. Takođe se ne može ručno upisivati. FastComments.com može samo odlučiti kada da upiše u revizijski zapis. Međutim, možete ga čitati putem ovog API-ja.

Događaji u revizijskom zapisu ističu nakon dve godine.