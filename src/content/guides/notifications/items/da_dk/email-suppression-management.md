Når e-mails sendt af FastComments bliver afvist (bounce) eller markeret som spam af modtageren, tilføjer e-mailudbyderen den adresse til en blokeringsliste. FastComments synkroniserer disse blokeringslister dagligt, så der ikke sendes yderligere e-mails til adresser, der ikke kan modtage dem.

Brugere og moderatorer med blokerede e-mailadresser vil ikke modtage nogen e-mailnotifikationer, inklusive svarnotifikationer, mentions, administratoradvarsler og sammendragsemails. Et rødt "Email Suppressed"-mærke vises ved siden af berørte brugere og moderatorer i admin-UI'en.

#### Visning af blokerede e-mails

Lejeradministratorer med tilladelsen Manage Data kan se blokerede e-mails på siden
[Blokerede e-mails](https://fastcomments.com/auth/my-account/suppressed-emails), under Manage Data.

Siden viser en tabel over alle blokerede e-mailadresser, der er knyttet til din lejers brugere, moderatorer og kommentatorer. Du kan filtrere efter e-mailadresse ved hjælp af søgefeltet.

#### Fjernelse af en blokering

For at fjerne en blokering skal du klikke på **Remove**-knappen ved siden af posten i tabellen. Du føres til en bekræftelsesside, der viser detaljer om blokeringen. Klik på **Confirm Removal** for at fortsætte.

Når en blokering fjernes, kontakter FastComments e-mailudbyderen for at åbne adressen igen og rydder blokeringflaget fra alle tilknyttede bruger- og moderatoroptegnelser.

#### Rate Limits

For at forhindre misbrug er fjernelser underlagt begrænsninger:

- Hver e-mailadresse kan kun fjernes fra blokeringen én gang hver 30. dag.
- Hver lejer kan foretage op til 5 fjernelser pr. kalendermåned.

Hvis en blokering genopstår efter fjernelsen, betyder det, at e-mailadressen igen blev afvist (bounce) eller rapporteret som spam. I så fald bør det underliggende leveringsproblem løses, før du forsøger endnu en fjernelse.