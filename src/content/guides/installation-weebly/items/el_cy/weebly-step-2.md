---
Για να λειτουργήσει σωστά η ενσωμάτωση Weebly με το FastComments, πρέπει να προσθέσουμε **δύο** μικρά κομμάτια κώδικα.

Το πρώτο απόσπασμα είναι για να κρύψει το μήνυμα του Weebly "Comments are Closed", και το δεύτερο είναι για να φορτώσει πραγματικά το FastComments.

First, copy this small code snippet:

[inline-code-attrs-start title = 'Απόσπασμα Κώδικα Επικεφαλίδας FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Then, on the same settings page from `Step One`, click the `+` next to `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα Κώδικα Επικεφαλίδας Ανάρτησης</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Άνοιγμα Κώδικα Επικεφαλίδας Ανάρτησης" />
</div>

You should see a text box open like this:

<div class="screenshot white-bg">
    <div class="title">Κώδικας Επικεφαλίδας Ανάρτησης Ανοιχτός</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Κώδικας Επικεφαλίδας Ανάρτησης Ανοιχτός" />
</div>

Now let's paste in our code snippet:

<div class="screenshot white-bg">
    <div class="title">Απόσπασμα Κώδικα Επικεφαλίδας Επικολλημένο</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Απόσπασμα Κώδικα Επικεφαλίδας Επικολλημένο" />
</div>

Next up is the footer code to enable FastComments. Click the plus sign next to `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα Κώδικα Υποσέλιδου Ανάρτησης</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Άνοιγμα Κώδικα Υποσέλιδου Ανάρτησης" />
</div>

Copy this code snippet which is designed **specifically for Weebly**:

[inline-code-attrs-start title = 'Απόσπασμα Κώδικα Υποσέλιδου FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // αφαίρεση κουμπιού εμφάνισης σχολίων
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Now let's paste in our footer code:

<div class="screenshot white-bg">
    <div class="title">Κώδικας Υποσέλιδου Ανάρτησης Προστέθηκε</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Κώδικας Υποσέλιδου Ανάρτησης Προστέθηκε" />
</div>

That's it!

---