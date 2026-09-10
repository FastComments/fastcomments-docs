## Okidači

Okidači pokreću Zap kada se nešto dogodi u FastComments. Sva tri su trenutna: FastComments isporučuje događaj Zapieru putem webhook‑a u trenutku kada se dogodi. Ništa ne ispituje vaš nalog i ne troše se API krediti čekajući.

| Okidač | Okida se kada |
|---------|----------------|
| Novi komentar | Komentar je objavljen. Podrazumevano se okida samo odobreni, ne‑spam komentari. |
| Ažuriran komentar | Komentar je izmenjen, odobren, glasao se za njega, zakačen, zaključan ili na drugi način izmenjen. |
| Obrišite komentar | Komentar je obrisan. |

Svaki okidač vraća kompletan komentar: id, URL stranice i URL ID, ime i e‑mail komentatora, tekst komentara kao markdown i kao HTML, broj glasova, oznake odobrenja i spama, jezik, domen i sve spominjanja. Polja se podudaraju sa payload‑om webhook‑a dokumentovanim u odeljku Webhooks, Data Structures.

## Opcije

**Domen.** Svaki okidač ima opcioni filter domena, koji prikazuje domene konfigurisane na vašem nalogu. Ostavite prazno da primate događaje sa svakog domena.

**Uključi neodobrene i spam komentare.** Samo na okidaču Novi komentar. Komentari koji su zadržani za moderaciju ili označeni kao spam podrazumevano se preskaču. Kada se takav komentar kasnije odobri, okidač Ažuriran komentar se aktivira za njega, pa Zap koji treba da reaguje na svaki komentar koji postane vidljiv koristi Ažuriran komentar sa filterom na polju odobrenja.

## Kako isporuka funkcioniše

Uključivanje Zapa kreira pretplatu na webhook na vašem nalogu, vidljivu na stranici Webhooks sa izvorom **API**. Isključivanje Zapa uklanja je. Zapier‑ova ograničenja se primenjuju na broj događaja koje prihvata po minuti; FastComments ponovo pokušava isporuku koja ne uspe, uz sve veći interval, i onemogućava pretplatu koja ne uspeva šest dana. Onemogućenu pretplatu možete ponovo omogućiti sa stranice Webhooks, ili jednostavno isključiti i ponovo uključiti Zap da biste kreirali novu.

Nalog može imati najviše 50 API pretplata. Svaki Zap koji koristi FastComments okidač koristi jednu.