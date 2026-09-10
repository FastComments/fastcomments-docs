## Okidači

Okidači pokreću Zap kada se nešto dogodi u FastComments. Sva tri su trenutna: FastComments isporučuje događaj Zapieru putem webhooka u trenutku kada se dogodi. Ništa ne ispituje vaš račun i ne troše se API krediti čekajući.

| Okidač | Pokreće se kada |
|---------|-----------|
| Novi komentar | Komentar je objavljen. Prema zadanim postavkama okidač se aktivira samo za odobrene, ne‑spam komentare. |
| Ažurirani komentar | Komentar je uređen, odobren, glasao se za njega, zakačen, zaključan ili na drugi način promijenjen. |
| Izbrisani komentar | Komentar je izbrisan. |

Svaki okidač vraća cijeli komentar: id, URL stranice i URL ID, ime i e‑mail komentatora, tekst komentara kao markdown i kao HTML, broj glasova, oznake odobrenja i spama, jezik, domenu i sve spominjanja. Polja odgovaraju payloadu webhooka dokumentiranom pod Webhooks, Data Structures.

## Opcije

**Domain.** Svaki okidač ima opcionalni filter domene, koji navodi domene konfigurirane na vašem računu. Ostavite prazno da primate događaje sa svih domena.

**Include Unapproved and Spam Comments.** Samo na okidaču Novi komentar. Komentari koji su zadržani za moderaciju ili označeni kao spam po defaultu se preskaču. Kada takav komentar kasnije bude odobren, okidač Ažurirani komentar se aktivira za njega, pa Zap koji treba reagirati na svaki komentar koji postane vidljiv koristi Ažurirani komentar s filterom na polju odobrenja.

## Kako funkcionira isporuka

Uključivanje Zapa stvara pretplatu na webhook na vašem računu, vidljivu na stranici Webhooks s izvorom **API**. Isključivanje Zapa uklanja je. Zapierova vlastita ograničenja primjenjuju se na broj događaja koje prihvaća po minuti; FastComments ponavlja isporuku koja ne uspije, s rastućim odgodom, i onemogućuje pretplatu koja neprestano ne uspijeva šest dana. Onemogućenu pretplatu možete ponovo omogućiti s stranice Webhooks, ili jednostavno isključiti i ponovno uključiti Zap kako biste stvorili novu.

Račun može imati najviše 50 API pretplata. Svaki Zap koji koristi FastComments okidač koristi jednu.