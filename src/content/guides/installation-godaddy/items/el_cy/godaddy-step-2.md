Τώρα που έχετε προσθέσει ένα προσαρμοσμένο μπλοκ HTML, μπορούμε να προσθέσουμε τον κώδικα του widget FastComments.

**Χρησιμοποιήστε τον ακόλουθο κώδικα για Godaddy, όχι κώδικα από άλλες οδηγίες. Αυτός ο κώδικας είναι ειδικός για Godaddy.**

Αντιγράψτε τον ακόλουθο κώδικα:

[inline-code-attrs-start title = 'Απόσπασμα Κώδικα Σχολίων Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Αυτό το συγκεκριμένο απόσπασμα κώδικα έχει σχεδιαστεί ώστε να είναι συμβατό με το Godaddy και θα εμφανίζεται μόνο στις αναρτήσεις του ιστολογίου σας — όχι στην κεντρική σελίδα.

Τώρα επικολλήστε τον κώδικα στην περιοχή `Custom Code` που αναφέρθηκε στο `Step One`.

<div class="screenshot white-bg">
    <div class="title">Αντιγράψτε και επικολλήστε τον κώδικα</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Αντιγράψτε και επικολλήστε τον κώδικα" />
</div>

Κάντε κλικ στο κουμπί Done επάνω δεξιά:

<div class="screenshot white-bg">
    <div class="title">Κάντε κλικ στο κουμπί Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Κάντε κλικ στο κουμπί Done" />
</div>

Αυτό ήταν για το βήμα δύο!