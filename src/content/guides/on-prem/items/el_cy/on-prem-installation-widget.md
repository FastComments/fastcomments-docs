Τα αποσπάσματα κώδικα του frontend και οι βιβλιοθήκες για On-Prem είναι τα ίδια με το προϊόν SaaS. Ωστόσο, πρέπει να καθορίσετε `apiHost` και τη σωστή διαδρομή του script:

[inline-code-attrs-start title = 'Κώδικας σχολίων για On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... μπορείτε επίσης να περάσετε SSO payload κ.λπ.
    }];
</script>
[inline-code-end]

Το παραπάνω είναι ένα πολύ απλό παράδειγμα. Μπορούμε επίσης να χρησιμοποιήσουμε τις επίσημες (1st-party) βιβλιοθήκες για React, Angular, Vue, Svelte κ.λπ.

---