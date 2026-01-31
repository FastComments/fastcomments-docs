Δεν συνιστάται να προσθέσετε το FastComments μέσω του Page Builder του BigCommerce, καθώς τότε ο κώδικας πρέπει να προστεθεί χειροκίνητα σε κάθε επιθυμητή σελίδα.

Ωστόσο, αν το επιθυμείτε, πρέπει να χρησιμοποιήσετε το ακόλουθο απόσπασμα κώδικα. Αποσπάσματα κώδικα από άλλα σεμινάρια δεν θα λειτουργήσουν λόγω της φύσης του BigCommerce:

[inline-code-attrs-start title = 'Απόσπασμα Page Builder του BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

---