[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Standardmäßig verfolgt FastComments nicht, wer jeden Kommentar angesehen hat, und liefert keine Statistiken dazu.

Wir können diese Funktion jedoch aktivieren, und dann beginnt das System, zu verfolgen, wenn jeder Benutzer zu einem Kommentar scrollt.

Wenn dies geschieht, wird ein Zähler neben einem Augensymbol, das bei jedem Kommentar angezeigt wird, erhöht. Der Zähler wird live aktualisiert und gemäß der Locale des Benutzers abgekürzt.

Wir können dies aktivieren, indem wir das **enableViewCounts**‑Flag auf true setzen:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Aktivieren von Kommentar-Ansichtszählungen'; code-example-end]

Dies kann ohne Code auf der Widget‑Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Widget-Anpassungsseite mit dem aktivierten Kontrollkästchen für Ansichtszähler, sodass jeder Kommentar ein Augensymbol und eine Zahl anzeigt'; title='Aktivieren von Kommentar-Ansichtszählungen' app-screenshot-end]

Wir verfolgen die Benutzer‑ID*, die den Kommentar angesehen hat, sodass ein erneutes Ansehen des Kommentars ihn nicht erhöht. Wenn Sie den Kommentar nach zwei Jahren erneut ansehen, wird der Zähler stärker erhöht.

- *Hinweis: oder die anonyme Sitzungs‑ID, oder die IP des Benutzers als gehashter Wert.

---