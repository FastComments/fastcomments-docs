Τώρα μπορούμε να αντιγράψουμε το παρακάτω απόσπασμα κώδικα. Χρησιμοποιήστε το κουμπί αντιγραφής που εμφανίζεται στην πάνω δεξιά γωνία του αποσπάσματος.

Υπάρχουν μερικά πράγματα που μπορείτε να ρυθμίσετε στον κώδικα, δείτε τις γραμμές 4 έως 7.

[inline-code-attrs-start title = 'Κώδικας Squarespace για μεμονωμένη σελίδα'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // your account id
    }];
</script>
[inline-code-end]

Θα πρέπει να μοιάζει έτσι:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>

Τώρα κάντε κλικ στην αποθήκευση στην πάνω δεξιά γωνία.

Σημειώστε ότι η επιλογή `Preview in Safe Mode` δεν θα λειτουργήσει, αλλά το widget θα εμφανιστεί όταν επισκεφθείτε τον ιστότοπό σας.

Αν αντιμετωπίζετε προβλήματα, βεβαιωθείτε ότι κοντά στο κάτω μέρος δεν γράφει `"tenantId": "demo"`. Θα πρέπει να εμφανίζεται το tenant id σας αν είστε συνδεδεμένοι. Αν όχι, επικοινωνήστε με την υποστήριξη.