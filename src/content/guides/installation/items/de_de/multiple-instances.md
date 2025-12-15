Jede Instanz des Kommentar-Widgets ist isoliert. Aus diesem Grund unterstützt FastComments von Haus aus mehr als eine Instanz pro Seite oder mehrere Instanzen, die auf denselben Chat-Thread verweisen.

Im Fall der VanillaJS-Bibliothek müssen Sie das Kommentar-Widget einfach an verschiedene DOM-Knoten binden. Wenn Sie einfach den aktuellen Thread auf der Seite aktualisieren möchten, siehe [Kommentar-Threads wechseln ohne die Seite neu zu laden](guide-customizations-and-configuration.html#switching-comment-threads);

### Synchronisierung des Authentifizierungsstatus über mehrere Instanzen

Betrachten wir das Beispiel einer benutzerdefinierten Single-Page-Anwendung, die eine Liste häufig gestellter Fragen mit eigenem Kommentar-Thread ist.

In diesem Fall haben wir mehrere Instanzen von FastComments gleichzeitig im DOM.

Das ist in Ordnung, aber es stellt einige Herausforderungen für die Benutzererfahrung dar.

Betrachten Sie diesen Ablauf:

1. Der Benutzer besucht die Seite mit einer Liste von Fragen, jede mit ihrem eigenen Kommentar-Widget.
2. Der Benutzer gibt seinen Benutzernamen und seine E-Mail-Adresse ein und hinterlässt eine Frage in einem der Threads.
3. Er sieht ein anderes FAQ-Element, zu dem er eine Frage hat.
4. Er möchte erneut kommentieren. Muss er seine E-Mail-Adresse und seinen Benutzernamen erneut eingeben?

In diesem Fall übernimmt FastComments die Synchronisierung des Authentifizierungsstatus über Widget-Instanzen für Sie. In Schritt vier wird der Benutzer bereits temporär authentifiziert sein, da er seinen Benutzernamen und seine E-Mail-Adresse auf derselben Seite eingegeben hat.
