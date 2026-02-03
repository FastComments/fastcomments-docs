FastComments ermöglicht es Ihnen, Erstkommentatoren dazu zu verpflichten, Ihre Nutzungsbedingungen (Terms of Service, TOS) zu akzeptieren, bevor sie einen Kommentar absenden.

Wenn aktiviert:
- **Anonyme Benutzer** sehen jedes Mal ein TOS-Kontrollkästchen, wenn sie kommentieren
- **Authentifizierte Benutzer** sehen das Kontrollkästchen nur bei ihrem ersten Kommentar oder wenn Sie Ihre TOS aktualisieren

### Nutzungsbedingungen aktivieren

Navigieren Sie zur Seite zur Anpassung des Widgets und aktivieren Sie das Kontrollkästchen "Annahme der Nutzungsbedingungen erforderlich":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Anpassung des TOS-Texts

Standardmäßig zeigt das Kontrollkästchen "Ich stimme den Nutzungsbedingungen und der Datenschutzerklärung zu" mit Links zu beiden Dokumenten an. Sie können diesen Text pro Gebietsschema anpassen, falls erforderlich:

1. Wählen Sie "Text pro Gebietsschema anpassen"
2. Wählen Sie das Gebietsschema aus dem Dropdown-Menü und geben Sie Ihren benutzerdefinierten Text ein

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Aktualisierung Ihrer Nutzungsbedingungen

Wenn Sie Ihre Nutzungsbedingungen aktualisieren, legen Sie das Datum "Zuletzt aktualisiert" fest. Benutzer, die den TOS vor diesem Datum akzeptiert haben, müssen erneut zustimmen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Funktionsweise

- Der Zeitstempel der Zustimmung zu den TOS wird pro Benutzer und pro Kommentar gespeichert
- Wenn ein Benutzer den TOS akzeptiert, wird das Datum in seinem Benutzerprofil (pro Mandant) gespeichert
- Wenn Sie ein "Zuletzt aktualisiert"-Datum festlegen, das nach dem Akzeptanzdatum des Benutzers liegt, muss er erneut zustimmen
- Für anonyme Benutzer, die nicht nachverfolgt werden können, erscheint das Kontrollkästchen bei jeder Kommentarabgabe