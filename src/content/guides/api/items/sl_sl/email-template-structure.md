Objekt `EmailTemplate` predstavlja konfiguracijo za prilagojen e-poštni predlog za najemnika.

Sistem bo izbral e-poštni predlog na podlagi:

- Njegovega identifikatorja vrste, imenovanega `emailTemplateId`. To so konstante.
- `domain`. Najprej bomo poskusili najti predlog za domeno, s katero je povezan ustrezni objekt (na primer `Comment`), in če ujemanja ne najdemo, bomo poiskali predlog, kjer je domain null ali `*`.

Struktura objekta `EmailTemplate` je naslednja:

[inline-code-attrs-start title = 'Struktura e-poštnega predloga'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** SAMO ZA BRANJE **/
    createdAt: string
    /** SAMO ZA BRANJE **/
    updatedAt: string
    /** SAMO ZA BRANJE **/
    updatedByUserId: string
    /** Domena, s katero naj bo predlog povezan. **/
    domain?: string | '*' | null
    /** Vsebina e-poštnega predloga v sintaksi EJS. **/
    ejs: string
    /** Preslikava prepisanih prevodnih ključev na vrednosti za vsako podprto lokalizacijo. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Objekt, ki predstavlja kontekst za izris predloga. **/
    testData: object
}
[inline-code-end]

### Opombe

- Veljavne vrednosti `emailTemplateId` lahko pridobite prek končne točke `/definitions`.
- Končna točka `/definitions` vsebuje tudi privzete prevode in testne podatke.
- Predlogi se ne bodo shranili, če je struktura ali testni podatki neveljavni.