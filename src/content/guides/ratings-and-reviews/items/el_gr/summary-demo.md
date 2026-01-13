Εδώ είναι μια μικρή επίδειξη του widget σύνοψης:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // remove query params
    });
</script>

### Κρυφή μνήμη

Σημειώστε ότι οι συνοψίσεις αποθηκεύονται στην κρυφή μνήμη για 30 δευτερόλεπτα, ή πέντε λεπτά αν υπάρχει μεγάλος αριθμός κριτικών. Εξαιτίας αυτού, η κριτική σας ενδέχεται να μην εμφανιστεί αμέσως στη σύνοψη, αλλά αυτό
μας επιτρέπει να μειώσουμε το κόστος παροχής του widget σύνοψης.