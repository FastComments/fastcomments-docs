Κάθε widget έχει τη δική του ετικέτα. Όλα αποδεχθούν **extra** ορίσματα λέξεων‑κλειδιών, τα οποία ενσωματώνονται στη διαμόρφωση του widget όπως είναι (χρησιμοποιήστε κλειδιά camelCase) για οτιδήποτε δεν καλύπτεται από τα ονομαστικά ορίσματα παρακάτω.

| Ετικέτα | Widget |
|---|---|
| `{% fastcomments %}` | Σχόλια |
| `{% fastcomments_live_chat %}` | Ζωντανή συνομιλία |
| `{% fastcomments_comment_count %}` | Ένδειξη αριθμού σχολίων |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Μαζικοί μετρητές σχολίων |
| `{% fastcomments_collab_chat target="#el" %}` | Συνεργατική (ενσωματωμένη) συνομιλία |
| `{% fastcomments_image_chat target="#el" %}` | Συνομιλία σχολιασμού εικόνας |
| `{% fastcomments_recent_comments %}` | Πρόσφατα σχόλια |
| `{% fastcomments_recent_discussions %}` | Πρόσφατες συζητήσεις |
| `{% fastcomments_reviews_summary %}` | Περίληψη αξιολογήσεων |
| `{% fastcomments_top_pages %}` | Σελίδες με τις περισσότερες συζητήσεις |
| `{% fastcomments_user_activity user_id="..." %}` | Ροή δραστηριότητας χρήστη |

Τα ονομαστικά ορίσματα αντιστοιχούν στα κλειδιά camelCase της διαμόρφωσης του widget:

| Παράμετρος | Κλειδί ρυθμίσεων | Ετικέτες |
|---|---|---|
| `url_id` | `urlId` | σχόλια, ζωντανή συνομιλία, ένδειξη αριθμού, συνεργατική/εικόνα συνομιλία, πρόσφατα σχόλια, περίληψη αξιολογήσεων |
| `url` | `url` | σχόλια, ζωντανή συνομιλία, συνεργατική/εικόνα συνομιλία |
| `readonly` | `readonly` | σχόλια, ζωντανή συνομιλία, συνεργατική/εικόνα συνομιλία |
| `locale` | `locale` | σχόλια, ζωντανή συνομιλία, συνεργατική/εικόνα συνομιλία, δραστηριότητα χρήστη |
| `has_dark_background` | `hasDarkBackground` | όλα |
| `default_sort_direction` | `defaultSortDirection` | σχόλια, ζωντανή συνομιλία, συνεργατική/εικόνα συνομιλία |
| `number_only` | `numberOnly` | ένδειξη αριθμού |
| `is_live` | `isLive` | ένδειξη αριθμού |
| `count` | `count` | πρόσφατα σχόλια, πρόσφατες συζητήσεις |
| `target` | (querySelector, not sent) | συνεργατική συνομιλία, εικόνα συνομιλία |
| `chat_square_percentage` | `chatSquarePercentage` | εικόνα συνομιλία |
| `user_id` | `userId` | δραστηριότητα χρήστη |

Παραδείγματα:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Η Συνεργατική συνομιλία προσδένεται σε ένα υπάρχον στοιχείο στη σελίδα #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Μαζικοί μετρητές: τοποθετήστε δείκτες, στη συνέχεια ένας μαζικός φορτωτής τα γεμίζει όλα #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```