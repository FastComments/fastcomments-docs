Τώρα που έχετε προσθέσει ένα προσαρμοσμένο μπλοκ HTML, μπορούμε να προσθέσουμε τον κώδικα του widget FastComments.

**Χρησιμοποιήστε τον ακόλουθο κώδικα για το Godaddy, όχι κώδικα από άλλα σεμινάρια. Αυτός ο κώδικας είναι ειδικός για το Godaddy.**

Αντιγράψτε τον ακόλουθο κώδικα:

[inline-code-attrs-start title = 'Απόσπασμα κώδικα σχολίων για Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Αυτό το συγκεκριμένο απόσπασμα κώδικα έχει σχεδιαστεί για να είναι συμβατό με το Godaddy και θα εμφανίζεται επίσης μόνο στις αναρτήσεις του ιστολογίου σας - όχι στην αρχική σελίδα.

Τώρα επικολλήστε τον κώδικα στην περιοχή `Custom Code` που αναφέρεται στο `Step One`.

<div class="screenshot white-bg">
    <div class="title">Αντιγραφή και Επικόλληση του Κώδικα</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Αντιγραφή και Επικόλληση του Κώδικα" />
</div>

Κάντε κλικ στο Done στην επάνω δεξιά γωνία:

<div class="screenshot white-bg">
    <div class="title">Κάντε κλικ στο Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Κάντε κλικ στο Done" />
</div>

Αυτό ήταν για το βήμα δύο!

---