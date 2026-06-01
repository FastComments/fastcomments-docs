For at vise en kommentaroptælling ved siden af mange sider på én gang (et blogindeks, en sektionsliste), skal du bruge bulk-antal-widgeten. Den henter alle optællinger på siden i én enkelt anmodning. Der er to dele: en markør ved siden af hvert element, og ét init-opkald efter listen.

I en listeskabelon (`layouts/_default/list.html`):

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

`count-marker.html` gengiver `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, og bruger samme identifikator som kommentar-widgeten bruger for den side (dens `urlId`, eller dens permalink når ingen `urlId` er sat), så optællingerne stemmer overens med de faktiske tråde. `bulk-count.html` udsender den enkelte anmodning, der udfylder dem.

Hvis du skriver markørerne selv (for eksempel i en sides Markdown), brug shortcoden til i stedet at udsende init-opkaldet:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```