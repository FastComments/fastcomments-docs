Når e-mails sendt af FastComments afvises (bounce) eller markeres som spam af modtageren, tilføjer e-mail-udbyderen adressen til en
suppressionsliste. FastComments synkroniserer disse suppressionslister dagligt, så der ikke sendes flere e-mails til adresser, der
ikke kan modtage dem.

Brugere og moderatorer med undertrykte e-mailadresser modtager ingen e-mailnotifikationer, herunder svarnotifikationer,
omtale-notifikationer, admin-advarsler og digest-e-mails. Et rødt "Email Suppressed"-mærke vises ved siden af berørte brugere
og moderatorer i admin-brugergrænsefladen.

#### Visning af undertrykte e-mails

Tenant-administratorer med Manage Data-tilladelsen kan se undertrykte e-mails på siden
[Suppressed Emails](https://fastcomments.com/auth/my-account/suppressed-emails) under Manage Data.

Siden viser en tabel over alle undertrykte e-mailadresser tilknyttet din tenants brugere, moderatorer og kommentatorer.
Du kan filtrere efter e-mailadresse via søgefeltet.

#### Fjernelse af en suppression

For at fjerne en suppression skal du klikke på knappen **Remove** ved siden af posten i tabellen. Du føres til en bekræftelsesside,
der viser detaljerne for suppressionen. Klik på **Confirm Removal** for at fortsætte.

Når en suppression fjernes, kontakter FastComments e-mail-udbyderen for at fjerne blokeringen af adressen og nulstiller
suppressionsflaget fra alle tilknyttede bruger- og moderatorposter.

#### Hastighedsbegrænsninger

For at forhindre misbrug er fjernelser hastighedsbegrænsede:

- Hver e-mailadresse kan kun frigøres fra suppression én gang hver 30. dag.
- Hver tenant kan udføre op til 5 fjernelser pr. kalendermåned.

Hvis en suppression viser sig igen efter fjernelse, betyder det, at e-mailadressen er afvist (bounce) eller igen er blevet rapporteret som spam. I så fald
bør det underliggende leveringsproblem løses, før man forsøger en ny fjernelse.