The front end code snippets and libraries for On-Prem are the same as the SaaS product. However, you must specify `apiHost` and the correct script path:

[inline-code-attrs-start title = 'Κώδικας σχολίων για On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... μπορείτε επίσης να περάσετε δεδομένα SSO κ.λπ.
    });
</script>
[inline-code-end]

Το παραπάνω είναι ένα πολύ απλό παράδειγμα. Μπορούμε επίσης να χρησιμοποιήσουμε τις επίσημες βιβλιοθήκες για React, Angular, Vue, Svelte κ.λπ.