[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments ist dafür ausgelegt, angepasst zu werden. Das Kommentierungs‑Widget selbst läuft aus Sicherheitsgründen in einem iframe, sodass Sie für benutzerdefiniertes Styling einen von zwei Ansätzen verfolgen müssen.

Der erste, einfachste Ansatz, den wir bevorzugen, ist die Nutzung der [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

Auf der Widget‑Anpassungsseite finden Sie den Abschnitt „Show Advanced Options“, darunter ein Bereich mit der Bezeichnung „Custom CSS“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Custom CSS-Editor unter "Show Advanced Options" auf der Widget-Anpassungsseite'; title='Custom CSS Eingabebereich' app-screenshot-end]

Dieser Ansatz hat einige Vorteile:
1. Das eingegebene CSS wird vor dem Versand an den Nutzer minifiziert, und die Formatierung bleibt in der Bearbeitungsoberfläche konsistent.
2. Sie erhalten alle Vorteile der Widget‑Anpassungsoberfläche, zum Beispiel das einfache Anpassen des Kommentierungs‑Widgets für verschiedene Websites.
3. Wenn wir Änderungen am Kommentierungs‑Widget vornehmen, wird Ihr benutzerdefiniertes Styling im Rahmen unseres Release‑Prozesses getestet.

Der zweite Ansatz besteht darin, den **customCSS**‑Parameter in der Widget‑Konfiguration anzugeben, wie folgt:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Custom CSS übergeben'; code-example-end]

Allerdings hat dies *Einschränkungen*:
1. Es gibt ein Limit, wie viel benutzerdefiniertes CSS übergeben werden kann, bevor unsere Server die Anfrage aufgrund der Header‑Größe ablehnen.
2. Sie müssen das benutzerdefinierte CSS in Ihrer Infrastruktur und Ihrem Build‑System verwalten. Das kann auch ein Vorteil sein.
3. Es entsteht ein zusätzlicher Aufwand, das benutzerdefinierte CSS in diesem Anwendungsfall **zweimal** über das Netzwerk zu senden, da es zuerst an unsere Server und dann zurück in den iframe‑Inhalt gesendet werden muss. Bei den meisten Payload‑Größen ist dies jedoch nicht bemerkbar.
4. Eine gängige Optimierung ist das Minifizieren des CSS, um die Größe im Netzwerk zu reduzieren; bei diesem Ansatz müssen Sie das jedoch selbst erledigen.
5. Ihr benutzerdefiniertes CSS wird nicht getestet, wenn wir Änderungen vornehmen.

### External CSS Files

Sie können das Widget anweisen, eine externe Datei mit `@import` zu laden!

Es wird empfohlen, das `@import` in einer Anpassungs‑Regel zu platzieren. Auf diese Weise können wir, falls wir jemals Änderungen am Kommentierungs‑Widget vornehmen müssen, unsere Automatisierungstools nutzen, um Ihre Einrichtung zu überprüfen. Zum Beispiel würden Sie in der Widget‑Anpassungs‑UI eine Anpassungs‑Regel erstellen, auf **Advanced** klicken und im Feld **Custom CSS** eingeben:

    @import url(https://example.com/styles.css);

#### In Code - Nicht empfohlen

Sie können auch eine externe CSS‑Datei über die `customCSS`‑Eigenschaft laden:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Externe CSS-Datei'; code-example-end]

Beachten Sie jedoch, dass Ihr CSS in diesem Fall nicht von uns getestet werden kann. 

### User Profile Modal Styling

Benutzerprofil‑Modale können ebenfalls mit benutzerdefiniertem CSS gestaltet werden. Damit das benutzerdefinierte Styling auf Benutzerprofile angewendet wird, muss jeder CSS‑Selektor mit `.user-profile` prefixed werden. Ohne dieses Präfix wird das Styling für Benutzerprofil‑Modale ignoriert.

Zum Beispiel:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'CSS für Benutzerprofil'; code-example-end]

### Backwards Compatibility

Bei FastComments wissen wir, dass unsere Kunden das Kommentierungs‑Widget anpassen. Das ist beabsichtigt – das Letzte, was wir wollen, ist, dass unser Produkt Design‑Inkonsistenzen in Ihrem Produkt verursacht.

Da dies ein wichtiger Teil unseres Produkts ist, haben wir eine Build‑Pipeline, die es uns ermöglicht, Änderungen am Kommentierungs‑Widget pro Kunde bei jedem Release zu prüfen.

Wenn wir kleinere Probleme finden, aktualisieren wir Ihr Konto, um einen reibungslosen Release zu gewährleisten. Wenn wir größere, brechende Änderungen sehen, können wir den Release stoppen.