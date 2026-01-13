[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Standardmäßig verfolgt FastComments nicht, wer jeden Kommentar angesehen hat, und stellt dazu keine Statistiken bereit.

Wir können diese Funktion jedoch aktivieren, und das System beginnt dann zu erfassen, wenn jeder Benutzer zu einem Kommentar scrollt.

In diesem Fall wird neben einem Augensymbol an jedem Kommentar eine Zählung erhöht. Die Zählung wird live aktualisiert und entsprechend der Lokalisierung des Benutzers abgekürzt.

Wir können dies aktivieren, indem wir das Flag **enableViewCounts** auf true setzen:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Dies kann ohne Code auf der Widget-Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Wir erfassen die Benutzer-ID*, die den Kommentar angesehen hat, damit beim erneuten Anzeigen des Kommentars die Zählung nicht erhöht wird. Wenn Sie den Kommentar jedoch erst nach zwei Jahren erneut ansehen, wird die Zählung wieder erhöht.

- *Hinweis: oder die anonyme Sitzungs-ID, oder die IP des Benutzers als gehashter Wert.