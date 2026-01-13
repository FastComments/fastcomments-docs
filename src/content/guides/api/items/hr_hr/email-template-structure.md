Objekt `EmailTemplate` predstavlja konfiguraciju za prilagođeni predložak e-pošte za zakupnika.

Sustav će odabrati predložak e-pošte za upotrebu putem:

- Njegovog identifikatora tipa, koji nazivamo `emailTemplateId`. To su konstante.
- Polja `domain`. Najprije ćemo pokušati pronaći predložak za domenu s kojom je povezani objekt (poput `Comment`) povezan, a ako se podudaranje ne pronađe tada ćemo pokušati pronaći predložak gdje je domain null ili `*`.

Struktura objekta `EmailTemplate` je sljedeća:

[inline-code-attrs-start title = 'Struktura predloška e-pošte'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Domena s kojom bi predložak trebao biti povezan. **/
    domain?: string | '*' | null
    /** Sadržaj predloška e-pošte u EJS sintaksi. **/
    ejs: string
    /** Mapa zamjenskih prijevoda (ključ → vrijednost) za svaku podržanu lokalizaciju. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Objekt koji predstavlja kontekst za renderiranje predloška. **/
    testData: object
}
[inline-code-end]

### Bilješke

- Valjane vrijednosti `emailTemplateId` možete dobiti s endpointa `/definitions`.
- Endpoint `/definitions` također uključuje zadane prijevode i testne podatke.
- Predlošci će propasti pri spremanju ako struktura ili testni podaci nisu valjani.