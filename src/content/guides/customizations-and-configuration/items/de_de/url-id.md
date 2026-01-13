[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Beim Anzeigen eines Kommentarthreads oder beim Verfassen eines Kommentars muss FastComments wissen, zu welcher Seite, welchem Artikel oder Produkt
diese Kommentare gehören.

Dafür verwenden wir etwas, das wir "URL ID" nennen. Dabei handelt es sich entweder um einen Bezeichner, wie eine Zeichenfolge oder eine Zahl, oder um eine URL.

Standardmäßig, wenn Sie das urlId nicht angeben, wird es zur Seiten-URL. Wir nehmen die aktuelle Seiten-URL und bereinigen sie, um
häufige Marketing-Parameter oder Tracking-Bezeichner zu entfernen.

Im Falle von Integrationen durch Dritte, wie WordPress, verwendet unser Plugin in der Regel den Bezeichner, der die aktuell betrachtete Information darstellt,
als URL ID, zum Beispiel die Artikel-/Seiten-ID.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Eines der Dinge, auf das wir in diesem Dokument häufig verweisen, ist die <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget-Anpassungsoberfläche</a>.

Diese Oberfläche kann verwendet werden, um viele Änderungen am Kommentar-Widget vorzunehmen, ohne Code zu benutzen.

Beim Erstellen einer Anpassungsregel möchten wir oft, dass sie auf alle Seiten unserer Website angewendet wird. In manchen Fällen möchten wir das Kommentar-Widget jedoch auf einer bestimmten Seite anpassen, entweder um individuelles Styling anzuwenden oder um Kommentare für diese bestimmte Seite anonym zu machen. Sie könnten zum Beispiel auch Live-Kommentare auf einigen Seiten sofort anzeigen lassen, während sie auf anderen Seiten unter Benachrichtigungstasten verborgen werden.

Das alles ist über das URL ID-Eingabefeld auf dieser Seite möglich, das wie folgt aussieht:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

Der Wert in diesem Feld sollte mit dem *urlId*-Parameter übereinstimmen, der an das Kommentar-Widget übergeben wird. Wenn Ihre Anpassungsregel *urlId*-unabhängig sein soll, lassen Sie dieses Feld leer oder geben Sie * ein.

Ab 2023 nimmt das `URL ID`-Feld in der Widget-Anpassung jetzt auch Muster auf! Zum Beispiel können Sie
`*/blog/*` haben, um Styling speziell für Ihren Blog hinzuzufügen, und `*/store/*`, um Styling speziell für Ihren Shop zu haben,
während die gleiche Domain verwendet wird.

### Fallstricke

1. Wenn Ihre Seite Hash-Parameter hat (wie example.com#page-1) – dies wird standardmäßig Teil der URL ID.
2. Während Migrationen, zum Beispiel von WordPress zu Gatsby, müssen Sie möglicherweise die URL ID-Kommentarwerte nach der anfänglichen Migration übertragen. Kontaktieren Sie uns dafür.