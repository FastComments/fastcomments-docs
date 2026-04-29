Wird ausgelöst, wenn ein Benutzer seinen ersten Kommentar auf dieser Site (Ihrem Tenant) abgibt. Dies geschieht **einmal pro Benutzer** - nachfolgende Kommentare desselben Benutzers lösen es nicht erneut aus.

### Kontext, den der Agent erhält

- Den neuen Kommentar.
- Optionaler Thread-/Benutzerverlauf-/Seitenkontext, wie konfiguriert.

Wenn der Benutzerverlaufskontext aktiviert ist, ist die Liste der letzten Kommentare des Benutzers natürlich leer (oder enthält nur diesen einen), aber der Vertrauensfaktor und das Alter des Kontos werden ausgefüllt.

### Wichtige Hinweise

- "Erster Kommentar auf dieser Site" bezieht sich auf den **Tenant**, nicht site-weit über FastComments. Ein Benutzer mit Kommentaren auf anderen FastComments-Sites löst diesen Trigger dennoch beim ersten Posting auf Ihrer Seite aus.
- Der Trigger wird nur für Benutzer mit einer userId ausgelöst. Anonyme, nicht verifizierte Kommentare ohne stabile userId lösen ihn nicht aus.
- Der Trigger wird ausgelöst, wenn der Kommentar genehmigt/sichtbar ist (nicht beim ersten Absenden). Änderungen oder Moderatoraktionen an Erstkommentaren lösen ihn nicht erneut aus.

### Häufige Verwendungszwecke

- **Begrüßung** - die [Welcome Greeter template](#template-welcome-greeter) ist um diesen Trigger herum aufgebaut.
- **Onboarding** - sende eine [DM-Warnung](#tool-warn-user) (hier als freundlicher Hinweis statt als Warnung verwendet), die den Benutzer auf die Community-Richtlinien hinweist.
- **Benachrichtigung für Prüfer** - wenn Sie möchten, dass ein Mensch jeden Erstkommentar eines neuen Kommentators prüft, kann [`mark_comment_reviewed`](#tools-overview) sie zur Überprüfung kennzeichnen.