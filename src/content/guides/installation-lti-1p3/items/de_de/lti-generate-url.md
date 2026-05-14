#### Navigieren Sie zur LTI 1.3-Konfiguration

Melden Sie sich bei FastComments an und gehen Sie zu <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">Ihrer LTI 1.3-Konfigurationsseite</a>.

If your account doesn't yet have LTI access, you'll see "LTI not enabled for this account" - contact support to enable it on your plan.

#### Plattform auswählen (optional)

Unter **Dynamische Registrierungs-URL generieren**, verwenden Sie das **Plattform**-Dropdown, um FastComments mitzuteilen, mit welchem LMS Sie sich verbinden:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Andere LTI 1.3-Plattform

Sie können es auch auf **Automatische Erkennung** belassen. Die Plattform wird während der Registrierung aus der openid-configuration Ihres LMS ausgelesen; das Dropdown setzt nur das Anzeigelabel für die resultierende Konfiguration.

#### URL generieren

Klicken Sie auf **URL generieren**. FastComments erstellt ein einmaliges Registrierungstoken und zeigt Ihnen eine URL an, die wie folgt aussieht:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopieren Sie sie. Diese URL:

- Ist **einmalig** - sobald Ihr LMS sie erfolgreich aufruft, wird das Token verbraucht.
- Läuft nach **30 Minuten** ab, wenn sie nicht verwendet wird.
- Sollte privat aufbewahrt werden - jeder mit der URL kann innerhalb dieser 30 Minuten ein Tool für Ihren Mandanten registrieren.

#### Bestehende Konfigurationen

Sobald eine Registrierung erfolgreich abgeschlossen ist, erscheint die neue Konfiguration in der Tabelle **Bestehende Konfigurationen** auf derselben Seite, mit ihrer Plattform, dem Issuer, der Client ID und dem Status. Sie können Konfigurationen aus dieser Tabelle löschen, wenn Sie sie jemals abmelden müssen.