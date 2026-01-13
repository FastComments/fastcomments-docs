[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments ist so konzipiert, dass es anpassbar ist. Das Kommentierungs-Widget läuft aus Sicherheitsgründen in einem iframe, daher müssen Sie für benutzerdefiniertes Styling eine von zwei Vorgehensweisen verwenden.

Die erste, einfachste Vorgehensweise – und von uns bevorzugt – ist die Verwendung der [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

In der Widget-Anpassungsseite sehen Sie den Abschnitt „Erweiterte Optionen anzeigen“ („Show Advanced Options“), unter dem es einen Bereich mit der Bezeichnung „Benutzerdefiniertes CSS“ („Custom CSS“) gibt:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Diese Vorgehensweise hat einige Vorteile:
1. Das eingegebene CSS wird vor dem Versand an den Nutzer minimiert, und das Format wird in der Bearbeitungsoberfläche konsistent beibehalten.
2. Sie erhalten alle Vorteile der Widget-Anpassungsoberfläche, zum Beispiel die einfache Möglichkeit, das Kommentierungs-Widget für verschiedene Seiten unterschiedlich anzupassen.
3. Wenn wir Änderungen am Kommentar-Widget vornehmen, wird Ihr benutzerdefiniertes Styling im Rahmen unseres Release-Prozesses getestet.

Die zweite Vorgehensweise besteht darin, den Parameter **customCSS** in der Widget-Konfiguration anzugeben, wie folgt:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Dies hat jedoch *Einschränkungen*:
1. Es gibt eine Begrenzung dafür, wie viel benutzerdefiniertes CSS übergeben werden kann, bevor unsere Server die Anfrage aufgrund der Header-Größe ablehnen.
2. Sie müssen das benutzerdefinierte CSS in Ihrer Infrastruktur und Ihrem Build-System verwalten. Das kann ebenso ein Vorteil wie ein Nachteil sein.
3. In diesem Anwendungsfall entsteht ein zusätzlicher Overhead, da das benutzerdefinierte CSS **zweimal** über das Netzwerk gesendet werden muss: einmal an unsere Server und dann wieder in den iframe-Inhalt. Bei den meisten Payload-Größen ist dies jedoch nicht wahrnehmbar.
4. Eine gängige Optimierung besteht darin, das CSS zu minimieren, um dessen Größe im Netzwerk zu reduzieren; bei dieser Vorgehensweise müssen Sie das jedoch selbst übernehmen.
5. Ihr benutzerdefiniertes CSS wird bei unseren Änderungen nicht getestet.

### Externe CSS-Dateien

Sie können dem Widget mitteilen, eine externe Datei zu laden, indem Sie `@import` verwenden!

Es wird empfohlen, das `@import` in eine Anpassungsregel zu setzen. Auf diese Weise können wir, falls wir jemals Änderungen am Kommentar-Widget vornehmen müssen, unsere Automatisierungstools verwenden, um Ihre Konfiguration zu überprüfen. Sie würden also zum Beispiel eine Anpassungsregel in der Widget-Anpassungsoberfläche erstellen, auf `Advanced` klicken und im Feld `Custom CSS` eingeben:

    @import url(https://example.com/styles.css);

#### Im Code - Nicht empfohlen

Sie können eine externe CSS-Datei auch über die Eigenschaft `customCSS` laden:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Beachten Sie jedoch, dass Ihr CSS in diesem Fall von uns nicht getestet werden kann. 

### Styling des Benutzerprofil-Modals

Benutzerprofil-Modals können ebenfalls mit benutzerdefiniertem CSS gestaltet werden. Damit das benutzerdefinierte Styling jedoch auf Benutzerprofile angewendet wird, müssen alle CSS-Selektoren mit `.user-profile` vorangestellt sein. Ohne dieses Präfix wird benutzerdefiniertes Styling für Benutzerprofil-Modals ignoriert.

Zum Beispiel:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Abwärtskompatibilität

Bei FastComments wissen wir, dass unsere Kunden das Kommentierungs-Widget anpassen. Das ist beabsichtigt – das Letzte, was wir wollen, ist, dass unser Produkt Design-Inkonsistenzen in Ihrem Produkt verursacht.

Da dies ein wichtiger Bestandteil unseres Produkts ist, verfügen wir über eine Build-Pipeline, die es uns ermöglicht, Änderungen am Kommentar-Widget pro Kunde bei jedem Release zu prüfen.

Wenn wir kleinere Probleme feststellen, werden wir Ihr Konto aktualisieren, um einen reibungslosen Release sicherzustellen. Wenn wir schwerwiegende, inkompatible Änderungen feststellen, erlaubt uns das, den Release zu stoppen.

---