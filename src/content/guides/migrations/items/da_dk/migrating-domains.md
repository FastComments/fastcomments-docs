FastComments giver dig en automatiseret måde at migrere dine kommentarer på tværs af domæner.

Domænemigreringen kræver blot et `from`- og et `to`-domæne.

Dette **flytter** kommentarer, det kopierer dem ikke. Hvis du ønsker at kopiere kommentarer, så kontakt os.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Domænemigrationsværktøj med felterne for fra- og til-domæne samt antallet af migrerede kommentarer'; title='Migrering af domæner' app-screenshot-end]

Dette er også nyttigt for eksempel, hvis en del af din migration til FastComments involverer at migrere fra en anden udbyder, så dine kommentarimportdata kan indeholde data, der skal migreres. I så fald kan du køre importen og derefter domænemigreringen.

### Overvågning af fremdrift

Domænemigrationsværktøjet bruger det samme FastComments jobbehandlingssystem som de andre datastyringsværktøjer.

Der kan være en forsinkelse, før din migration starter. Dette er normalt, da systemet periodisk tjekker for nye jobs, der skal behandles.

Mens jobbet kører, vil det vise antallet af kommentarer, der er fundet til migration, samt antallet af kommentarer, der er migreret indtil videre.