---
Objekat `EmailTemplate` predstavlja konfiguraciju za prilagođeni email šablon, za tenant.

Sistem će izabrati email šablon za upotrebu na osnovu:

- Njegovog identifikatora tipa, koji nazivamo `emailTemplateId`. To su konstante.
- `domain`. Prvo ćemo pokušati da pronađemo šablon za domen sa kojim je povezan relevantan objekat (npr. `Comment`), a ako se poklapanje ne pronađe pokušaćemo da pronađemo šablon gde je domain null ili `*`.

Struktura objekta `EmailTemplate` je sledeća:

[inline-code-attrs-start title = 'Struktura email šablona'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** SAMO ZA ČITANJE **/
    createdAt: string
    /** SAMO ZA ČITANJE **/
    updatedAt: string
    /** SAMO ZA ČITANJE **/
    updatedByUserId: string
    /** Domen sa kojim bi šablon trebalo da bude povezan. **/
    domain?: string | '*' | null
    /** Sadržaj email šablona u EJS sintaksi. **/
    ejs: string
    /** Mapa zamenjenih prevodnih ključeva na vrednosti, za svaki podržani locale. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Objekat koji predstavlja kontekst renderovanja šablona. **/
    testData: object
}
[inline-code-end]

### Napomene

- Možete dobiti validne `emailTemplateId` vrednosti sa `/definitions` endpoint-a.
- `/definitions` endpoint takođe uključuje podrazumevane prevode i test podatke.
- Šabloni neće moći da se sačuvaju ako struktura ili test podaci nisu validni.

---