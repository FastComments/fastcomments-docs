[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments ist darauf ausgelegt, angepasst zu werden. Das Kommentar-Widget selbst läuft aus Sicherheitsgründen in einem iframe, daher müssen Sie
benutzerdefiniertes Styling nach einer von zwei Methoden anwenden.

Die erste, einfachste Methode – und von uns bevorzugt – ist die Verwendung der [Widget-Anpassungsseite](https://fastcomments.com/auth/my-account/customize-widget).

Auf der Widget-Anpassungsseite finden Sie den Abschnitt „Show Advanced Options", unter dem sich ein Bereich mit der Bezeichnung „Custom CSS" befindet:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Custom CSS Editor unter Show Advanced Options auf der Widget-Anpassungsseite'; title='Custom CSS Eingabebereich' app-screenshot-end]

Diese Methode hat einige Vorteile:
1. Das eingegebene CSS wird minifiziert, bevor es an den Benutzer gesendet wird, und die Formatierung bleibt in der Bearbeitungs-UI konsistent.
2. Sie profitieren von allen Vorteilen der Widget-Anpassungs-UI, z. B. der einfachen Anpassung des Kommentar-Widgets für verschiedene Websites.
3. Wenn wir Änderungen am Kommentar-Widget vornehmen, wird Ihr benutzerdefiniertes Styling als Teil unseres Release-Prozesses getestet.

Die zweite Methode besteht darin, den Parameter **customCSS** in der Widget-Konfiguration anzugeben, wie folgt:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Benutzerdefiniertes CSS übergeben'; code-example-end]

Dies hat jedoch *Einschränkungen*:
1. Es gibt ein Limit, wie viel benutzerdefiniertes CSS übergeben werden kann, bevor unsere Server die Anfrage aufgrund der Header-Größe ablehnen.
2. Sie müssen das benutzerdefinierte CSS in Ihrer Infrastruktur und Ihrem Build-System verwalten. Dies kann auch ein Vorteil statt ein Nachteil sein.
3. In diesem Anwendungsfall entsteht ein zusätzlicher Overhead, da das benutzerdefinierte CSS **zweimal** über das Netzwerk gesendet wird – zunächst an unsere Server und dann zurück im iframe-Inhalt. Bei den meisten Payload-Größen ist dies jedoch nicht merkbar.
4. Eine gängige Optimierung ist das Minifizieren des CSS, um seine Größe über das Netzwerk zu reduzieren – bei dieser Methode müssen Sie das jedoch selbst übernehmen.
5. Ihr benutzerdefiniertes CSS wird nicht getestet, wenn wir Änderungen vornehmen.

### Externe CSS-Dateien

Sie können das Widget anweisen, eine externe Datei über `@import` zu laden!

Es wird empfohlen, das `@import` in einer Anpassungsregel zu platzieren. So können wir, falls wir jemals eine Änderung am Kommentar-Widget vornehmen müssen, unsere Automatisierungs-Tools verwenden, um Ihr Setup zu überprüfen. Sie würden also beispielsweise eine Anpassungsregel in der Widget-Anpassungs-UI erstellen, auf `Advanced` klicken und in `Custom CSS` Folgendes eingeben:

    @import url(https://example.com/styles.css);

#### Im Code – Nicht empfohlen

Sie können eine externe CSS-Datei auch über die Eigenschaft `customCSS` laden:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Externe CSS-Datei'; code-example-end]

Beachten Sie jedoch, dass Ihr CSS in diesem Fall nicht von uns getestet werden kann.

### Styling des Benutzerprofil-Modals

Benutzerprofil-Modals können ebenfalls mit benutzerdefiniertem CSS gestaltet werden. Um jedoch sicherzustellen, dass benutzerdefiniertes Styling auf Benutzerprofile angewendet wird, müssen alle CSS-Selektoren mit `.user-profile` vorangestellt werden. Ohne dieses Präfix wird benutzerdefiniertes Styling für Benutzerprofil-Modals ignoriert.

Zum Beispiel:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'Benutzerprofil-CSS'; code-example-end]

### Abwärtskompatibilität

Bei FastComments wissen wir, dass unsere Kunden das Kommentar-Widget anpassen. Das ist beabsichtigt – das Letzte, was wir wollen, ist, dass unser Produkt Design-Inkonsistenzen in Ihrem Produkt verursacht.

Da dies ein wichtiger Teil unseres Produkts ist, verfügen wir über eine Build-Pipeline, die es uns ermöglicht, Änderungen am Kommentar-Widget bei jedem Release pro Kunde zu überprüfen.

Wenn wir kleinere Probleme finden, aktualisieren wir Ihr Konto, um einen reibungslosen Release sicherzustellen. Bei größeren, breaking Changes können wir den Release anhalten.