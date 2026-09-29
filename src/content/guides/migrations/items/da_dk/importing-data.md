While FastComments Support can help with migrations, most can be performed and monitored easily without any intervention of support staff.

We natively support importing exports from the following providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via pluginet, eller en XML eller CSV export)
- AnyComment (Via WordPress Import/Export)

By navigating [her](https://fastcomments.com/auth/my-account/manage-data/import) can we upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments importside med udbyderudvælgelse og filuploadfelter for en eksportfil'; title='Importsideformularen' app-screenshot-end]

### Overvågning af import

FastComments bruger et jobbehandlingssystem til at behandle import og eksport. Når systemet har hentet dit job, vil det periodisk rapportere jobstatus i import- eller eksport‑UI'en.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Import side, der viser et kørende importjob og den status, der rapporteres af jobbehandlingssystemet'; title='Importjobstatus' app-screenshot-end]

Bemærk, at status for import og eksport kan ses af alle administratorer i kontoen.

Hvis dit job fejler, vil det ikke automatisk blive genstartet. Importen skal forsøges igen. Hvis en import eller eksport fejler, bliver vores systemadministratorer automatisk underrettet. Hvis vi identificerer et problem, kontakter vi dig for at se, om vi kan hjælpe.

### Kørsel af import igen

Under visse migrationer er det nødvendigt at køre importen flere gange. For eksempel er det almindeligt at foretage en første migrationsrunde til test, og derefter køre importen igen med de nyeste data, før man skifter til produktion.

Genimport af det samme indhold **vil ikke oprette dubletter**.

### Datasikkerhed og udløb

Importfiler er på ingen måde tilgængelige via eksterne anmodninger, og importfiler slettes fra vores system, så snart importen er fuldført.

---