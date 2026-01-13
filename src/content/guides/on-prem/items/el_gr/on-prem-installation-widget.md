---
Τα αποσπάσματα κώδικα του front end και οι βιβλιοθήκες για On-Prem είναι τα ίδια με το προϊόν SaaS. Ωστόσο, πρέπει να ορίσετε το `apiHost` και τη σωστή διαδρομή του script:

[inline-code-attrs-start title = 'Κώδικας σχολίων για On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... μπορείτε επίσης να περάσετε φορτίο SSO κ.λπ.
    });
</script>
[inline-code-end]

Το παραπάνω είναι ένα πολύ απλό παράδειγμα. Μπορούμε επίσης να χρησιμοποιήσουμε τις επίσημες βιβλιοθήκες React, Angular, Vue, Svelte κ.ά.

---