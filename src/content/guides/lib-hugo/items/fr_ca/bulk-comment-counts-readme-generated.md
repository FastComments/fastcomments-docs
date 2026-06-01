Pour afficher le nombre de commentaires à côté de plusieurs pages à la fois (un index de blog, une liste de section), utilisez le widget de comptage en masse. Il récupère tous les nombres de commentaires présents sur la page en une seule requête. Il y a deux éléments : un marqueur à côté de chaque élément, et un appel d'initialisation après la liste.

Dans un modèle de liste (`layouts/_default/list.html`):

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

`count-marker.html` rend `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, en utilisant le même identifiant que le widget de commentaires utilise pour cette page (son `urlId`, ou son permalink lorsque aucun `urlId` n'est défini), afin que les nombres correspondent aux fils réels. `bulk-count.html` émet la requête unique qui les remplit.

Si vous écrivez vous-même les marqueurs (par exemple dans le Markdown d'une page), utilisez le shortcode pour émettre l'appel d'initialisation à la place :

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```