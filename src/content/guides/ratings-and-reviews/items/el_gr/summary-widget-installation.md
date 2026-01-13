Παρακάτω βρίσκεται ο κώδικας Vanilla JS για την εγκατάσταση του Widget Σύνοψης. Η βιβλιοθήκη React διαθέτει επίσης αυτό το widget.

[inline-code-attrs-start title = 'Εγκατάσταση Widget Σύνοψης'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Το widget θα εντοπίσει αυτόματα τις ερωτήσεις που θα εμφανιστούν στη σύνοψη βάσει της αντίστοιχης ρύθμισης του widget για εκείνη τη σελίδα/τον ιστότοπο.

Εάν χρειάζεστε το widget σε κάποια από τις άλλες βιβλιοθήκες μας που δεν το περιλαμβάνουν, ανοίξτε ένα αίτημα υποστήριξης ώστε να γνωρίζουμε να το προσθέσουμε.