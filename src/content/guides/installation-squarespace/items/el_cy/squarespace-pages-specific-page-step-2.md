Τώρα μπορούμε να αντιγράψουμε το ακόλουθο απόσπασμα κώδικα. Χρησιμοποιήστε το κουμπί αντιγραφής που εμφανίζεται πάνω δεξιά στο απόσπασμα.

Υπάρχουν μερικά πράγματα που μπορείτε να διαμορφώσετε στον κώδικα, δείτε τις γραμμές 4 έως 7.

[inline-code-attrs-start title = 'Κώδικας για μεμονωμένη σελίδα Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // το αναγνωριστικό του λογαριασμού σας
    }];
</script>
[inline-code-end]

Θα πρέπει να μοιάζει με αυτό:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>

Τώρα κάντε κλικ στο κουμπί Αποθήκευση πάνω δεξιά.

Σημειώστε ότι η επιλογή `Preview in Safe Mode` δεν θα λειτουργήσει, αλλά το widget θα εμφανιστεί όταν επισκεφθείτε τον ιστότοπό σας.

Αν αντιμετωπίζετε προβλήματα, βεβαιωθείτε ότι κοντά στο κάτω μέρος δεν λέει `"tenantId": "demo"`. Θα πρέπει να εμφανίζει το tenant id σας αν έχετε συνδεθεί. Αν όχι, επικοινωνήστε με την υποστήριξη.