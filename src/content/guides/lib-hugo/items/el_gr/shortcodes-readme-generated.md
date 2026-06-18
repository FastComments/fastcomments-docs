| Shortcode | Περιγραφή |
| --- | --- |
| `fastcomments` | Σχόλια σε νήματα με απαντήσεις, ψήφους και @αναφορές |
| `fastcomments-comment-count` | Αριθμός σχολίων για μία σελίδα |
| `fastcomments-comment-count-bulk` | Αριθμοί σχολίων για πολλές σελίδες σε ένα αίτημα (βλ. [Μαζικοί αριθμοί σχολίων](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Widget ζωντανής συνομιλίας |
| `fastcomments-collab-chat` | Συνεργατικά ενσωματωμένα σχόλια (απαιτεί `target`) |
| `fastcomments-image-chat` | Σχόλια σχολιασμού εικόνας (απαιτεί `target`) |
| `fastcomments-recent-comments` | Πρόσφατα σχόλια σε όλο τον ιστότοπο |
| `fastcomments-recent-discussions` | Πρόσφατα ενεργά νήματα συζήτησης |
| `fastcomments-reviews-summary` | Σύνοψη αξιολογήσεων με αστέρια |
| `fastcomments-top-pages` | Πιο συζητημένες σελίδες |
| `fastcomments-user-activity-feed` | Ροή δραστηριότητας ανά χρήστη (απαιτεί `userId`) |

### Παραδείγματα

Αριθμός σχολίων εντός κειμένου:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Ζωντανή συνομιλία:

```text
\{{< fastcomments-live-chat >}}
```

Συνεργατική συνομιλία, στοχεύοντας ένα στοιχείο περιεχομένου με επιλογέα CSS:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Συνομιλία εικόνας, στοχεύοντας στοιχείο εικόνας με επιλογέα CSS:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Σύνοψη αξιολογήσεων:

```text
\{{< fastcomments-reviews-summary >}}
```

Ροή δραστηριότητας χρήστη:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```