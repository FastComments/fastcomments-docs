| Ετικέτα | Περιγραφή |
| --- | --- |
| `fastcomments` | Ζωντανά σχόλια με απαντήσεις, ψηφοφορία, διαχείριση και ενημερώσεις σε πραγματικό χρόνο |
| `fastcomments_comment_count` | Αριθμός σχολίων για την τρέχουσα σελίδα |
| `fastcomments_comment_count_bulk` | Αριθμοί σχολίων για πολλές σελίδες σε μια λίστα/σελίδα ευρετηρίου |
| `fastcomments_live_chat` | Widget συνομιλίας σε πραγματικό χρόνο |
| `fastcomments_collab_chat` | Συνεργατικός ενσωματωμένος σχολιασμός (σημειώσεις κειμένου) |
| `fastcomments_image_chat` | Σχόλια/σημειώσεις σε εικόνες |
| `fastcomments_recent_comments` | Πρόσφατα σχόλια σε όλο τον ιστότοπο |
| `fastcomments_recent_discussions` | Πρόσφατα ενεργά νήματα συζήτησης |
| `fastcomments_reviews_summary` | Περίληψη αξιολογήσεων με αστέρια |
| `fastcomments_top_pages` | Πιο συζητημένες σελίδες |
| `fastcomments_user_activity_feed` | Ροή δραστηριότητας ανά χρήστη |

### Παραδείγματα

```liquid
{% raw %}{# Αριθμός σχολίων. Το widget εμφανίζει τη δική του ετικέτα, π.χ. "0 comments" #}
{% fastcomments_comment_count %}

{# Ζωντανή συνομιλία #}
{% fastcomments_live_chat %}

{# Συνεργατική συνομιλία. Στοχεύστε το σε ένα στοιχείο περιεχομένου με έναν CSS selector #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Συνομιλία εικόνας. Στοχεύστε το σε ένα στοιχείο εικόνας με έναν CSS selector #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Περίληψη αξιολογήσεων #}
{% fastcomments_reviews_summary %}

{# Ροή δραστηριότητας χρήστη. Απαιτεί user id #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Μαζικοί αριθμοί σχολίων για ένα ευρετήριο ιστολογίου #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```