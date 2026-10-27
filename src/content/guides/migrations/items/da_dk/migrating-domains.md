FastComments giver en automatiseret måde for dig at migrere dine kommentarer på tværs af domæner.

Domænemigrationen kræver blot et `from` og et `to` domæne.

Dette **flytter** kommentarer, det kopierer dem ikke. Hvis du ønsker at kopiere kommentarer, så kontakt os.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Domænemigrationsværktøj med fra- og til-domænefelterne og antallet af kommentarer, der er migreret'; title='Migrering af domæner' app-screenshot-end]

Dette er også nyttigt for eksempel hvis en del af din migration til FastComments involverer at migrere fra en anden udbyder, så dine kommentarimport‑data kan indeholde data, der skal migreres. I så fald kan du køre importen og derefter domænemigrationen.

### Overvågning af fremdrift

Domænemigrationsværktøjet bruger det samme FastComments job‑behandlingssystem som de andre datastyringsværktøjer.

Der kan være en forsinkelse før din migration starter. Dette er normalt, da systemet periodisk tjekker for nye jobs, der skal behandles.

Når jobbet kører, vil det vise antallet af kommentarer, der er fundet til migration, og antallet der er migreret indtil videre.

### Migrering af domæner via API

Domæner kan også migreres med [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), ved at sætte `type` til `domain`. Fremdrift er tilgængelig fra [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), og migrationen kan fortrydes i 90 dage.

---