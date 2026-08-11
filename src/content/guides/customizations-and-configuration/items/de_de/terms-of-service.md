FastComments ermöglicht es Ihnen, von Erstkommentatoren die Zustimmung zu Ihren Nutzungsbedingungen zu verlangen, bevor sie einen Kommentar absenden.

Wenn aktiviert:
- **Anonyme Benutzer** sehen bei jedem Kommentar ein Nutzungsbedingungen‑Kontrollkästchen
- **Authentifizierte Benutzer** sehen das Kontrollkästchen nur bei ihrem ersten Kommentar oder wenn Sie Ihre Nutzungsbedingungen aktualisieren

### Konfiguration

Navigieren Sie zur Seite zur Widget‑Anpassung und aktivieren Sie das Kontrollkästchen „Zustimmung zu den Nutzungsbedingungen erforderlich“. Sobald es aktiviert ist, sehen Sie die folgenden Optionen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Nutzungsbedingungen-Panel, das den Textmodus-Auswahl und das Feld für das zuletzt aktualisierte Datum anzeigt'; title='Optionen für die Nutzungsbedingungen' app-screenshot-end]

- **TOS-Textmodus**: Standardmäßig zeigt das Kontrollkästchen „Ich stimme den Nutzungsbedingungen und der Datenschutzerklärung zu“ mit Links zu beiden Dokumenten an. Wählen Sie „Text pro Sprache anpassen“, um Ihren eigenen Text für jede Sprache bereitzustellen.
- **Datum der letzten Aktualisierung der Nutzungsbedingungen**: Wenn Sie Ihre Nutzungsbedingungen aktualisieren, setzen Sie dieses Datum. Benutzer, die vor diesem Datum zugestimmt haben, müssen erneut zustimmen.

### So funktioniert es

- Der Zeitstempel der Zustimmung zu den Nutzungsbedingungen wird pro Benutzer und pro Kommentar gespeichert
- Wenn ein Benutzer den Nutzungsbedingungen zustimmt, wird das Datum in seinem Benutzerprofil (pro Mandant) aufgezeichnet
- Wenn Sie ein „Letztes Aktualisierungsdatum“ festlegen, das nach dem Zustimmungsdatum des Benutzers liegt, muss er erneut zustimmen
- Für anonyme Benutzer, die nicht nachverfolgt werden können, erscheint das Kontrollkästchen bei jeder Kommentarabgabe