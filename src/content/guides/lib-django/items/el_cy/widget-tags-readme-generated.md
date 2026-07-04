Κάθε στοιχείο έχει τη δική του ετικέτα. Όλα αποδέχονται τα ορίσματα‑λέξεις‑κλειδιά `**extra`, τα οποία ενσωματώνονται στη διαμόρφωση του στοιχείου ακριβώς όπως είναι (χρησιμοποιήστε κλειδιά camelCase) για οτιδήποτε δεν καλύπτεται από τις ονομαστικές παραμέτρους παρακάτω.

| Ετικέτα | Στοιχείο |
|---|---|
| `{% fastcomments %}` | Σχόλια |
| `{% fastcomments_live_chat %}` | Ζωντανή συνομιλία |
| `{% fastcomments_comment_count %}` | Σήμα μετρητή σχολίων |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Μαζικοί μετρητές σχολίων |
| `{% fastcomments_collab_chat target="#el" %}` | Συνεργατική (ενσωματωμένη) συνομιλία |
| `{% fastcomments_image_chat target="#el" %}` | Συνομιλία επεξηγήσεων εικόνας |
| `{% fastcomments_recent_comments %}` | Πρόσφατα σχόλια |
| `{% fastcomments_recent_discussions %}` | Πρόσφατες συζητήσεις |
| `{% fastcomments_reviews_summary %}` | Περίληψη αξιολογήσεων |
| `{% fastcomments_top_pages %}` | Σελίδες με τις περισσότερες συζητήσεις |
| `{% fastcomments_user_activity user_id="..." %}` | Ροή δραστηριότητας χρήστη |

Οι ονομαστικές παράμετροι αντιστοιχούν στα κλειδιά διαμόρφωσης camelCase του στοιχείου:

| Παράμετρος | Κλειδί διαμόρφωσης | Ετικέτες |
|---|---|---|
| `url_id` | `urlId` | Σχόλια, Ζωντανή συνομιλία, Σήμα μετρητή σχολίων, Συνεργατική/εικονογραφική συνομιλία, Πρόσφατα σχόλια, Περίληψη αξιολογήσεων |
| `url` | `url` | Σχόλια, Ζωντανή συνομιλία, Συνεργατική/εικονογραφική συνομιλία |
| `readonly` | `readonly` | Σχόλια, Ζωντανή συνομιλία, Συνεργατική/εικονογραφική συνομιλία |
| `locale` | `locale` | Σχόλια, Ζωντανή συνομιλία, Συνεργατική/εικονογραφική συνομιλία, Ροή δραστηριότητας χρήστη |
| `has_dark_background` | `hasDarkBackground` | Όλα |
| `default_sort_direction` | `defaultSortDirection` | Σχόλια, Ζωντανή συνομιλία, Συνεργατική/εικονογραφική συνομιλία |
| `number_only` | `numberOnly` | Σήμα μετρητή σχολίων |
| `is_live` | `isLive` | Σήμα μετρητή σχολίων |
| `count` | `count` | Πρόσφατα σχόλια, Πρόσφατες συζητήσεις |
| `target` | (querySelector, not sent) | Συνεργατική συνομιλία, Εικονογραφική συνομιλία |
| `chat_square_percentage` | `chatSquarePercentage` | Εικονογραφική συνομιλία |
| `user_id` | `userId` | Ροή δραστηριότητας χρήστη |

Παραδείγματα:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Η συνεργατική συνομιλία προσαρμόζεται σε ένα υπάρχον στοιχείο στη σελίδα #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Μαζικοί μετρητές: τοποθετήστε δείκτες, κατόπιν ένας μαζικός φορτωτής τα συμπληρώνει όλα #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```