Παρακάτω είναι ο κώδικας Vanilla JS για την εγκατάσταση του Widget Περίληψης. Η βιβλιοθήκη React διαθέτει επίσης αυτό το widget.

[inline-code-attrs-start title = 'Εγκατάσταση Widget Περίληψης'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Το widget θα εντοπίζει αυτόματα τις ερωτήσεις που θα εμφανιστούν στη σύνοψη βάσει της αντίστοιχης ρύθμισης του widget για αυτή τη σελίδα/ιστότοπο.

Εάν χρειάζεστε το widget σε κάποια από τις άλλες βιβλιοθήκες μας που δεν το περιλαμβάνουν, υποβάλετε ένα αίτημα υποστήριξης ώστε να ξέρουμε να το προσθέσουμε.