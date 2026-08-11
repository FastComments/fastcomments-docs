[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Beim Rendern eines Kommentar‑Threads oder beim Verfassen eines Kommentars muss FastComments wissen, zu welcher Seite, welchem Artikel oder welchem Produkt diese Kommentare gehören.

Dazu verwenden wir etwas, das wir die „URL ID“ nennen. Es ist entweder ein Bezeichner, wie ein String oder eine Zahl, oder eine URL.

Standardmäßig, wenn Sie keine urlId angeben, wird die Seiten‑URL verwendet. Wir nehmen die aktuelle Seiten‑URL und bereinigen sie, um gängige Marketing‑Parameter oder Tracking‑Kennungen zu entfernen.

Im Fall von Drittanbieter‑Integrationen, wie WordPress, verwendet unser Plugin in der Regel den Bezeichner, der die aktuell angezeigte Information repräsentiert, als URL ID, zum Beispiel die Artikel‑/Seiten‑ID.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definieren einer benutzerdefinierten URL ID'; code-example-end]

Ein Element, das wir in diesem Dokument häufig referenzieren, ist die <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget‑Anpassungs‑UI</a>.

Diese UI kann verwendet werden, um viele Änderungen am Kommentar‑Widget vorzunehmen, ohne Code zu benutzen.

Beim Erstellen einer Anpassungsregel möchten wir oft, dass sie für alle Seiten unserer Website gilt. In manchen Fällen wollen wir jedoch das Kommentar‑Widget auf einer bestimmten Seite anpassen, entweder um benutzerdefiniertes Styling anzuwenden oder um Kommentare für diese Seite anonym zu machen. Man könnte zum Beispiel auch Live‑Kommentare sofort auf einigen Seiten anzeigen lassen, während sie auf anderen hinter Benachrichtigungs‑Buttons verborgen werden.

All dies ist über das URL‑ID‑Eingabefeld auf dieser Seite möglich, das wie folgt aussieht:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='URL‑ID‑Feld, das verwendet wird, um eine Anpassungsregel auf eine Seite zu beschränken oder auf ein Muster wie */blog/*'; title='URL‑ID‑Eingabe auf der Widget‑Anpassungsseite' app-screenshot-end]

Der Wert in diesem Feld sollte mit dem *urlId*-Parameter übereinstimmen, der an das Kommentar‑Widget übergeben wird. Wenn Ihre Anpassungsregel *urlId*-unabhängig sein soll, lassen Sie dieses Feld leer oder geben Sie * ein.

Ab 2023 akzeptiert das Feld `URL ID` in der Widget‑Anpassung nun auch Muster! Zum Beispiel können Sie `*/blog/*` verwenden, um Styling speziell für Ihren Blog hinzuzufügen, und `*/store/*`, um Styling speziell für Ihren Store zu haben, und das alles bei Verwendung derselben Domain.

### Fallstricke

1. Wenn Ihre Seite Hash‑Parameter hat (wie example.com#page-1) – wird dies standardmäßig Teil der URL ID.
2. Während Migrationen, zum Beispiel von WordPress zu Gatsby, müssen Sie möglicherweise die URL‑ID‑Kommentarwerte nach der ersten Migration migrieren. Dafür kontaktieren Sie uns bitte.

---