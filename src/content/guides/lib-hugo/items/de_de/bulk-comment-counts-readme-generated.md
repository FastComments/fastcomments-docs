Um neben vielen Seiten gleichzeitig die Anzahl der Kommentare anzuzeigen (z. B. in einem Blog-Index oder einer Bereichsliste), verwenden Sie das Bulk-Count-Widget. Es ruft alle Zähler auf der Seite in einer einzigen Anfrage ab. Es besteht aus zwei Teilen: einem Marker neben jedem Eintrag und einem Initialisierungsaufruf nach der Liste.

In einer Listen-Vorlage (`layouts/_default/list.html`):

```go-html-template
<ul>
  \{{ range .Pages }}
    <li>
      <a href="\{{ .RelPermalink }}">\{{ .Title }}</a>
      \{{ partial "fastcomments/count-marker.html" . }}
    </li>
  \{{ end }}
</ul>
\{{ partial "fastcomments/bulk-count.html" (dict "page" .) }}
```

`count-marker.html` gibt `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>` aus und verwendet denselben Identifikator, den das Kommentar-Widget für diese Seite benutzt (sein `urlId`, oder dessen Permalink, wenn kein `urlId` gesetzt ist), sodass die Zähler mit den echten Threads übereinstimmen. `bulk-count.html` sendet die einzelne Anfrage, die diese Werte befüllt.

Wenn Sie die Marker selbst schreiben (z. B. im Markdown einer Seite), verwenden Sie stattdessen den Shortcode, um den Initialisierungsaufruf zu erzeugen:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```