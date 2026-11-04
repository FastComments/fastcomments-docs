Υπάρχουν μερικά endpoints για τη λήψη των μετρήσεων, ανάλογα με το τι θέλετε και αν θέλετε να τις λάβετε από browser, server ή μέσω του
API SDK.

## Δημόσιες Μετρήσεις Σχολίων

Μπορείτε να λάβετε τις δημόσιες μετρήσεις σχολίων χρησιμοποιώντας τα widgets παραπάνω ή τα APIs που χρησιμοποιούν. Αυτά τα APIs παραμένουν αμετάβλητα από το 2019 και
δεν θα αλλάξουν ποτέ.

[inline-code-attrs-start title = 'Endpoint Μεμονωμένης Μέτρησης'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Αυτό θα επιστρέψει μια δομή όπως:

[inline-code-attrs-start title = 'Απάντηση Endpoint Μεμονωμένης Μέτρησης'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

Η ιδιότητα `postfix` περιλαμβάνεται πάντα.

[inline-code-attrs-start title = 'Endpoint Μαζικής Μέτρησης'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Αυτό θα επιστρέψει μια δομή όπως:

[inline-code-attrs-start title = 'Απάντηση Endpoint Μαζικής Μέτρησης'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

Το αντικείμενο `counts` συμπληρώνεται μόνο για σελίδες που έχουν μετρήσεις. Ο χάρτης `translations` είναι πάντα παρών καθώς χρησιμοποιείται για το widget.

### Συμπεριφορά Δημόσιων Endpoints / Προσωρινή Αποθήκευση

Τα δημόσια endpoints διαθέτουν μηχανισμό προσωρινής αποθήκευσης 60 δευτερολέπτων για την αντιμετώπιση αιχμών επισκεψιμότητας. Εσωτερικά πρόκειται για LRU cache στη μνήμη ανά νήμα στον server,
επομένως ενδέχεται να δείτε τις μετρήσεις να αλλάζουν ελαφρώς (να ανεβαίνουν και να κατεβαίνουν προσωρινά) όταν οι χρήστες αφήνουν πολλά σχόλια.

Τα δημόσια endpoints επιστρέφουν πάντα το *συνολικό* πλήθος σχολίων, όχι το πλήθος των ριζικών σχολίων.

### APIs / SDK Επικεντρωμένα στον Server

Ο τρόπος για να λάβετε σχόλια από τον server σας είναι να καλέσετε το [Pages API](/guide-api.html#page-structure) και να λάβετε το αντικείμενο της σελίδας, το οποίο περιέχει το συνολικό πλήθος σχολίων
και το πλήθος των ριζικών σχολίων. Παρέχουμε SDKs που σας επιτρέπουν να καλέσετε αυτό το API χωρίς να κατασκευάσετε χειροκίνητα το αίτημα API, και παρέχουν τυποποιημένες τιμές επιστροφής.