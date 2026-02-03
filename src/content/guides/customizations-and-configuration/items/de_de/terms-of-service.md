---
FastComments ermöglicht es Ihnen, Erstkommentatoren zu verpflichten, Ihre Nutzungsbedingungen zu akzeptieren, bevor sie einen Kommentar absenden.

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Konfiguration

Navigieren Sie zur Seite zur Anpassung des Widgets und aktivieren Sie das Kontrollkästchen "Akzeptieren der Nutzungsbedingungen erforderlich". Sobald aktiviert, sehen Sie die folgenden Optionen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS-Textmodus**: Standardmäßig zeigt das Kontrollkästchen "I agree to the Terms of Service and Privacy Policy" mit Links zu beiden Dokumenten an. Wählen Sie "Customize text per locale", um Ihren eigenen Text für jede Sprache bereitzustellen.
- **Datum der letzten Aktualisierung der Nutzungsbedingungen**: Wenn Sie Ihre Nutzungsbedingungen aktualisieren, legen Sie dieses Datum fest. Benutzer, die vor diesem Datum zugestimmt haben, müssen erneut zustimmen.

### Funktionsweise

- Der Zeitstempel der TOS-Akzeptanz wird pro Benutzer und pro Kommentar gespeichert
- Wenn ein Benutzer den TOS zustimmt, wird das Datum in seinem Benutzerprofil (pro Mandant) aufgezeichnet
- Wenn Sie ein "Zuletzt aktualisiert"-Datum festlegen, das nach dem Zustimmungsdatum des Benutzers liegt, muss dieser erneut zustimmen
- Bei anonymen Benutzern, die nicht nachverfolgt werden können, erscheint das Kontrollkästchen bei jeder Kommentarabgabe

---