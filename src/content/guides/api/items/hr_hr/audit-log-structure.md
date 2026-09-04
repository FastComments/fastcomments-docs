An `AuditLog` je objekt koji predstavlja revizijski događaj za najamnike koji imaju pristup ovoj značajci.

Struktura objekta AuditLog je sljedeća:

[inline-code-attrs-start title = 'Struktura AuditLoga'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Tko je izvršio događaj. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Preglednik koji je izvršio događaj, kada je došao iz preglednika. **/
    ua?: string;
    /** Hash sesije iz koje je došao događaj, za povezivanje radnji jedne osobe. Nikada sama sesija. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** ID objekta na kojem je izvršen događaj, nasuprot tko ga je izvršio. **/
    targetId?: string;
    /** Ljudski čitljiva oznaka za taj objekt, npr. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` i `targetLabel` opisuju na čemu je događaj izvršen; `userId` i `username` opisuju tko ga je izvršio. Za ažuriranja, `objectDetails.changes` sadrži mapu `{field: {from, to}}` koja prikazuje što se zapravo promijenilo.

Revizijski zapis je nepromjenjiv. Također se ne može ručno zapisivati. FastComments.com može odlučiti kada zapisati u revizijski zapis. Međutim, možete ga čitati putem ovog API-ja.

Događaji u revizijskom zapisu istječu nakon dvije godine.