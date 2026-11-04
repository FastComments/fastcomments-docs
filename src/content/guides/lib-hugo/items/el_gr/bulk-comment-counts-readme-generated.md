Για να εμφανίσετε έναν μετρητή σχολίων δίπλα σε πολλές σελίδες ταυτόχρονα (ευρετήριο blog, λίστα ενότητας), χρησιμοποιήστε το widget μαζικής μέτρησης. Ανακτά όλες τις μετρήσεις της σελίδας σε ένα μόνο αίτημα. Αποτελείται από δύο μέρη: έναν δείκτη δίπλα σε κάθε στοιχείο και μία κλήση αρχικοποίησης μετά τη λίστα.

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

Το `count-marker.html` αποδίδει `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, χρησιμοποιώντας το ίδιο αναγνωριστικό που χρησιμοποιεί το widget σχολίων για τη συγκεκριμένη σελίδα (το `urlId` της, ή το permalink της όταν δεν έχει οριστεί `urlId`), ώστε οι μετρήσεις να αντιστοιχούν στα πραγματικά νήματα συζητήσεων. Το `bulk-count.html` εκπέμπει το μοναδικό αίτημα που τις συμπληρώνει.

Αν γράφετε τους δείκτες μόνοι σας (για παράδειγμα στο Markdown μιας σελίδας), χρησιμοποιήστε το shortcode για να εκπέμψετε την κλήση αρχικοποίησης:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```