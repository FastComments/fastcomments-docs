Mens FastComments Support kan hjælpe med migrationer, kan de fleste udføres og overvåges nemt uden nogen indgriben
fra supportmedarbejdere.

Vi understøtter nativt import af eksporter fra følgende udbydere:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via plugin'et, eller en XML- eller CSV-eksport)
- AnyComment (Via WordPress Import/Export)

Ved at navigere [her](https://fastcomments.com/auth/my-account/manage-data/import) kan vi uploade filen, der indeholder dataene, som skal migreres.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments importside med valg af udbyder og felter til filupload af en eksportfil'; title='Importsidens formular' app-screenshot-end]

### Overvågning af imports

FastComments bruger et jobbehandlingssystem til at behandle imports og eksporter. Når systemet har hentet dit job, vil det
periodisk rapportere jobbets status i import- eller eksport-UI'en.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Importside, der viser et kørende importjob og status rapporteret af jobbehandlingssystemet'; title='Status for importjob' app-screenshot-end]

Bemærk, at status for imports og eksporter kan ses af alle administratorer på kontoen.

Hvis dit job fejler, vil det ikke automatisk blive genstartet. Importen skal forsøges igen. Hvis en import eller eksport fejler,
bliver vores systemadministratorer automatisk underrettet. Hvis vi identificerer et problem, kontakter vi dig for at se, om vi kan hjælpe.

### Kørsel af importen igen

Under nogle migrationer er det nødvendigt at køre importen flere gange. For eksempel er det almindeligt at lave en første
migrering til test, og derefter køre importen igen med de nyeste data, før man skifter kontakten.

Genimport af det samme indhold **vil ikke skabe dubletter**.

### Datasikkerhed og udløb

Importfiler er ikke tilgængelige via eksterne forespørgsler på nogen måde, og importfiler slettes fra vores system, så snart
importen er fuldført.