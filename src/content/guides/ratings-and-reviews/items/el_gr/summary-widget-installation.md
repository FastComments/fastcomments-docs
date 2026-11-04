Παρακάτω είναι ο κώδικας Vanilla JS για την εγκατάσταση του Summary Widget. Η βιβλιοθήκη React διαθέτει επίσης αυτό το widget.

[inline-code-attrs-start title = 'Εγκατάσταση Summary Widget'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Το widget θα εντοπίσει αυτόματα τις ερωτήσεις που θα εμφανιστούν στη σύνοψη με βάση τη αντίστοιχη διαμόρφωση widget για τη συγκεκριμένη σελίδα/ιστότοπο.

### Φιλτράρισμα Σχολίων ανά Βαθμολογία

Όταν το widget σχολίων για την ίδια σελίδα βρίσκεται επίσης στη σελίδα και είναι ρυθμισμένο να εμφανίζει την απάντηση κάθε σχολιαστή στο σχόλιό του, οι γραμμές της ανάλυσης γίνονται δυνατές για κλικ. Κάνοντας κλικ σε μια γραμμή, για παράδειγμα "5", το widget σχολίων φιλτράρεται στα σχόλια που έδωσαν αυτή την απάντηση. Κάνοντας ξανά κλικ στη γραμμή, ή στο X στο chip "Showing only" στο widget σχολίων, εμφανίζονται ξανά όλα τα σχόλια. Αυτό δεν απαιτεί καμία διαμόρφωση: το Summary Widget εντοπίζει το widget σχολίων με το ίδιο `tenantId` και `urlId`.

Αυτό λειτουργεί για ερωτήσεις τύπου αστέρια, thumbs, emoji, κλίμακα και απλή επιλογή. Η ανάλυση που εμφανίζει το widget σχολίων πάνω από το πλαίσιο σχολίων μπορεί να γίνει κλικ με τον ίδιο τρόπο.

Εάν χρειάζεστε το widget σε μία από τις άλλες βιβλιοθήκες μας που δεν το διαθέτει, υποβάλετε ένα support ticket ώστε να γνωρίζουμε ότι πρέπει να το προσθέσουμε.