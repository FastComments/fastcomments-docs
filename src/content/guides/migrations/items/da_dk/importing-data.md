---
Selvom FastComments Support kan hjælpe med migrationer, kan de fleste udføres og overvåges nemt uden nogen indblanding fra supportpersonale.

Vi understøtter indbygget import af eksporter fra følgende udbydere:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via pluginet)
- AnyComment (Via WordPress Import/Export)

Ved at navigere [her](https://fastcomments.com/auth/my-account/manage-data/import) kan vi uploade filen, der indeholder de data, der skal migreres.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Overvågning af importer

FastComments bruger et jobbehandlingssystem til at behandle importer og eksporter. Når systemet har taget dit job op, vil det periodisk rapportere status for jobbet i import- eller eksport-UI'en.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Bemærk, at status for importer og eksporter kan ses af alle administratorer på kontoen.

Hvis dit job fejler, vil det ikke blive genstartet automatisk. Importen skal forsøges igen. Hvis en import eller eksport fejler, bliver vores systemadministratorer automatisk underrettet. Hvis vi identificerer et problem, kontakter vi dig for at se, om vi kan hjælpe.

### Genkørsel af importen

Under nogle migrationer er det nødvendigt at køre importen flere gange. For eksempel er det almindeligt at lave en første gennemkørsel til test, og så køre importen igen med de nyeste data inden man skifter over.

Genimport af det samme indhold **vil ikke skabe dubletter**.

### Datasikkerhed og udløb

Importfiler er ikke tilgængelige via eksterne forespørgsler på nogen måde, og importfiler slettes fra vores system så snart importen er fuldført.

---