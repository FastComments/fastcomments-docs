Εδώ είναι μια μικρή επίδειξη του widget περίληψης:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // αφαίρεσε τις παραμέτρους ερωτήματος
    });
</script>

### Κρυφή μνήμη

Σημειώστε ότι οι περιλήψεις αποθηκεύονται στην κρυφή μνήμη για 30 δευτερόλεπτα, ή για πέντε λεπτά αν υπάρχει μεγάλος αριθμός κριτικών. Εξαιτίας αυτού, η κριτική σας ενδέχεται να μην εμφανιστεί αμέσως στην περίληψη, αλλά αυτό
μας επιτρέπει να μειώσουμε το κόστος παροχής του widget περίληψης.

---