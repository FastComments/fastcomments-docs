Τα αποσπάσματα κώδικα του front end και οι βιβλιοθήκες για On-Prem είναι τα ίδια με το προϊόν SaaS. Ωστόσο, πρέπει να ορίσετε `apiHost` και το σωστό script path:

[inline-code-attrs-start title = 'Κώδικας σχολίων για On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... μπορείτε επίσης να περάσετε payload SSO κ.λπ.
    }];
</script>
[inline-code-end]

Το παραπάνω είναι ένα πολύ απλό παράδειγμα. Θα μπορούσαμε επίσης να χρησιμοποιήσουμε τις 1st-party βιβλιοθήκες React, Angular, Vue, Svelte, κ.λπ.