FastComments tilbyder en automatiseret måde for dig at migrere dine kommentarer på tværs af domæner.

Domænemigreringen kræver blot et `from`- og et `to`-domæne.

Dette **flytter** kommentarer, det kopierer dem ikke. Hvis du ønsker at kopiere kommentarer, kontakt os.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; title='Migrating Domains' app-screenshot-end]

Dette er også nyttigt, for eksempel hvis en del af din migration til FastComments involverer migration fra en anden udbyder, så din kommentarimport
data kan indeholde data, der skal migreres. I så fald kan du først køre importen, og derefter domænemigreringen.

### Overvågning af fremdrift

Domænemigreringsværktøjet bruger det samme FastComments-jobbehandlingssystem som de andre datahåndteringsværktøjer.

Der kan være en forsinkelse, før din migration starter. Dette er normalt, da systemet periodisk kontrollerer efter nye job, der skal behandles.

Mens jobbet kører, vil det vise antallet af kommentarer, der er fundet til migration, og hvor mange der er migreret indtil videre.

---