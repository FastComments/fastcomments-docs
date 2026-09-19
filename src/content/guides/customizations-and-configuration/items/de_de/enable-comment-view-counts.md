[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Standardmäßig verfolgt FastComments nicht, wer jeden Kommentar angesehen hat, oder liefert irgendwelche Statistiken dazu.

Wir können jedoch diese Funktion aktivieren, und dann wird das System beginnen, zu verfolgen, wenn jeder Benutzer zu einem Kommentar scrollt.

Wenn dies geschieht, wird ein Zähler neben einem Augensymbol, das bei jedem Kommentar angezeigt wird, erhöht. Der Zähler wird live aktualisiert und gemäß der Locale des Benutzers abgekürzt.

Wir können dies aktivieren, indem wir das **enableViewCounts**-Flag auf true setzen:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Dies kann ohne Code auf der Widget-Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Widget-Anpassungsseite mit dem aktivierten Kontrollkästchen für Ansichtszähler, sodass jeder Kommentar ein Augensymbol und eine Zahl anzeigt'; title='Enabling Comment View Counts' app-screenshot-end]

Wir verfolgen die Benutzer-ID*, die den Kommentar für eine Woche angesehen hat, sodass ein erneutes Anzeigen des Kommentars innerhalb dieser Woche den Zähler nicht erhöht. Wenn Sie den Kommentar nach Ablauf der Woche erneut ansehen, wird der Zähler wieder erhöht.

- *Hinweis: oder die anonyme Sitzungs-ID, oder die IP des Benutzers als gehashten Wert.