Για να εμφανίσετε έναν μετρητή σχολίων δίπλα σε πολλές σελίδες ταυτόχρονα (ένας κατάλογος ιστολογίου, μια λίστα ενότητας), χρησιμοποιήστε το widget μαζικής μέτρησης. Ανακτά κάθε μετρητή στη σελίδα με ένα μόνο αίτημα. Υπάρχουν δύο κομμάτια: ένας δείκτης δίπλα σε κάθε στοιχείο, και μια κλήση init μετά τη λίστα.

Σε ένα template λίστας (`layouts/_default/list.html`):

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

`count-marker.html` αποδίδει `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, χρησιμοποιώντας το ίδιο αναγνωριστικό που χρησιμοποιεί το widget σχολίων για εκείνη τη σελίδα (το `urlId` του, ή το permalink όταν δεν έχει οριστεί `urlId`), ώστε οι μετρήσεις να ευθυγραμμίζονται με τα πραγματικά νήματα. Το `bulk-count.html` εκπέμπει το μοναδικό αίτημα που τα συμπληρώνει.

Αν γράψετε τους δείκτες μόνοι σας (για παράδειγμα στο Markdown μίας σελίδας), χρησιμοποιήστε το shortcode για να εκτελέσετε την κλήση init αντί αυτού:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```