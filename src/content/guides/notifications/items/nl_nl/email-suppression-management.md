Wanneer e-mails die door FastComments worden verzonden bounce'en of door de ontvanger als spam worden gemarkeerd, voegt de e-mailprovider dat adres toe aan een
suppressielijst. FastComments synchroniseert deze suppressielijsten dagelijks zodat er geen verdere e-mails naar adressen worden verzonden die
ze niet kunnen ontvangen.

Gebruikers en moderators met onderdrukte e-mailadressen ontvangen geen e-mailmeldingen, inclusief reactiemeldingen,
meldingen bij vermeldingen, beheerwaarschuwingen en samenvattingsmails. Een rood "E-mail onderdrukt" label verschijnt naast getroffen gebruikers
en moderators in de beheer-UI.

#### Onderdrukte e-mails bekijken

Tenant-beheerders met de machtiging 'Gegevens beheren' kunnen onderdrukte e-mails bekijken op de
[Onderdrukte e-mails](https://fastcomments.com/auth/my-account/suppressed-emails) pagina, onder Gegevens beheren.

De pagina toont een tabel met alle onderdrukte e-mailadressen die gekoppeld zijn aan de gebruikers, moderators en commentatoren van uw tenant.
U kunt filteren op e-mailadres met het zoekveld.

#### Een onderdrukking verwijderen

Om een onderdrukking te verwijderen, klikt u op de **Verwijderen** knop naast de vermelding in de tabel. U wordt naar een bevestigings
pagina gebracht die de details van de onderdrukking toont. Klik op **Bevestig verwijdering** om door te gaan.

Wanneer een onderdrukking wordt verwijderd, neemt FastComments contact op met de e-mailprovider om het adres te deblokkeren en wist het de onderdrukking
vlag uit alle gerelateerde gebruikers- en moderatorrecords.

#### Limieten

Om misbruik te voorkomen, zijn verwijderingen beperkt door limieten:

- Elk e-mailadres kan slechts eenmaal per 30 dagen worden opgeheven.
- Elke tenant kan tot 5 verwijderingen per kalendermaand uitvoeren.

Als een onderdrukking na verwijdering opnieuw verschijnt, betekent dit dat het e-mailadres opnieuw is gebounced of weer als spam is gerapporteerd. In dat geval
moet het onderliggende afleverbaarheidsprobleem worden opgelost voordat u een nieuwe verwijdering probeert.