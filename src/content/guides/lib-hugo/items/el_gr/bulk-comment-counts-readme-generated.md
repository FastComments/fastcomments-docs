Για να εμφανίσετε τον αριθμό σχολίων δίπλα σε πολλές σελίδες ταυτόχρονα (ένα ευρετήριο ιστολογίου, μια λίστα ενότητας), χρησιμοποιήστε το widget μαζικής καταμέτρησης. Αυτό ανακτά κάθε μέτρηση στη σελίδα με ένα μόνο αίτημα. Υπάρχουν δύο μέρη: ένας δείκτης δίπλα σε κάθε στοιχείο, και μία κλήση init μετά τη λίστα.

Σε ένα πρότυπο λίστας (`layouts/_default/list.html`):

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

`count-marker.html` renders `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, using the same identifier the comments widget uses for that page (its `urlId`, or its permalink when no `urlId` is set), so the counts line up with the real threads. `bulk-count.html` αποστέλλει το μοναδικό αίτημα που τις συμπληρώνει.

If you write the markers yourself (for example in a page's Markdown), use the shortcode to emit the init call instead:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```