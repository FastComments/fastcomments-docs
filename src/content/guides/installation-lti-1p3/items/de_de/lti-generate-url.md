#### Zur LTI 1.3-Konfiguration navigieren

Melden Sie sich bei FastComments an und gehen Sie zu <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">Ihrer LTI 1.3-Konfigurationsseite</a>.

Wenn Ihr Konto noch keinen LTI-Zugang hat, sehen Sie "LTI not enabled for this account" - kontaktieren Sie den Support, um es für Ihren Tarif zu aktivieren.

#### Plattform auswählen (Optional)

Unter **Generate a Dynamic Registration URL** verwenden Sie die **Platform**-Dropdownliste, um FastComments mitzuteilen, mit welchem LMS Sie sich verbinden:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Andere LTI 1.3-Plattform

Sie können es auch auf **Auto-detect** belassen. Die Plattform wird während der Registrierung aus der openid-configuration Ihres LMS ausgelesen; die Dropdownliste setzt nur das Anzeige-Label für die resultierende Konfiguration.

#### URL generieren

Klicken Sie auf **Generate URL**. FastComments erstellt ein einmaliges Registrierungs-Token und zeigt Ihnen eine URL, die wie folgt aussieht:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopieren Sie sie. Diese URL:

- Ist **einmal verwendbar** - sobald Ihr LMS sie erfolgreich aufruft, wird das Token verbraucht.
- Läuft nach **30 Minuten** ab, wenn sie nicht verwendet wird.
- Sollte privat gehalten werden - jeder mit der URL kann innerhalb dieser 30 Minuten ein Tool gegen Ihren Tenant registrieren.

#### Bestehende Konfigurationen

Sobald eine Registrierung erfolgreich abgeschlossen ist, erscheint die neue Konfiguration in der Tabelle **Existing Configurations** auf derselben Seite, mit ihrer Platform, Issuer, Client ID und Status. Sie können Konfigurationen aus dieser Tabelle löschen, falls Sie jemals die Registrierung aufheben müssen.