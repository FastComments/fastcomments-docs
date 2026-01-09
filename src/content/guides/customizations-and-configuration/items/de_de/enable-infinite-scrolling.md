[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Standardmäßig passt sich das FastComments-Widget in der Höhe an, um alle sichtbaren Kommentare anzuzeigen. Die Paginierung erfolgt über eine "Weiter anzeigen"-Schaltfläche am Ende der aktuellen Seite, da wir festgestellt haben, dass diese Interaktion für die meisten Benutzer am angenehmsten ist.

Es gibt jedoch Fälle, in denen unendliches Scrollen bevorzugt wird. Zum Beispiel verwenden wir diese Funktion in unserem Stream Chat-Produkt.

Wir können die "Weiter anzeigen"-Schaltflächen ausblenden und auf unendliches Scrollen umschalten, indem wir das Flag **enableInfiniteScrolling** auf true setzen:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Dafür ist außerdem das Hinzufügen von benutzerdefiniertem CSS erforderlich. Füge zum Beispiel benutzerdefiniertes CSS für den Selektor `.comments` hinzu, um das Scrollen zu ermöglichen:

[inline-code-attrs-start title = 'Unendliches Scrollen aktivieren'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Ein vollständiges, funktionierendes Beispiel wäre:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

Im obigen Beispiel verwenden wir die Eigenschaft `customCSS`; es wird jedoch empfohlen, stattdessen die Widget-Konfigurationsoberfläche aus Leistungsgründen zu verwenden. [Siehe die Dokumentation zu Custom CSS.](/guide-customizations-and-configuration.html#custom-css)