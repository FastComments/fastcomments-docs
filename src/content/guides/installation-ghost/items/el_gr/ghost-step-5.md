Στη συνέχεια πρέπει να εντοπίσουμε πού να προσθέσουμε τον κώδικα του widget του FastComments.com.

Αν χρησιμοποιείτε το προεπιλεγμένο θέμα `casper`, θα δείτε μια ενότητα σαν αυτή στη γραμμή `82`:

<div class="screenshot white-bg">
    <div class="title">Απενεργοποιημένη Ενότητα Σχολίων</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Απενεργοποιημένη Ενότητα Σχολίων" />
</div>

Αν χρησιμοποιείτε άλλα θέματα, δεν θα δείτε αυτό, και θα χρειαστεί να προσθέσετε αυτόν τον κώδικα μετά το τελευταίο `</section>`:

[inline-code-attrs-start title = 'Παράδειγμα Ενότητας'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Πρέπει να έχετε κάτι σαν αυτό έτοιμο:

<div class="screenshot white-bg">
    <div class="title">Πρότυπο Έτοιμο για Κώδικα Σχολίων</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Πρότυπο Έτοιμο για Κώδικα Σχολίων" />
</div>

Μόλις είστε έτοιμοι, αντιγράψτε τον κώδικα του widget του FastComments.com:

[inline-code-attrs-start title = 'Απόσπασμα Κώδικα Σχολίων FastComments.com για Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let simpleSSO = null;

        \{{#if access}}
            \{{#if @member}}
                simpleSSO = {
                    id: '\{{ @member.uuid }}',
                    email: '\{{@member.email}}',
                    username: '\{{@member.name}}',
                    avatar: '\{{ @member.avatar_image }}',
                    optedInNotifications: true,
                    optedInSubscriptionNotifications: true,
                    displayLabel: '\{{@member.labels}}'
                }
            \{{/if}}
        \{{/if}}

        FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        });
    })();
</script>
[inline-code-end]

...και θα πρέπει να μοιάζει έτσι:

<div class="screenshot white-bg">
    <div class="title">Προσθήκη Κώδικα Σχολίων FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Προσθήκη Κώδικα Σχολίων FastComments.com" />
</div>

Ο κώδικας ολοκληρώθηκε. Τώρα απλώς πρέπει να επανεισάγουμε το θέμα μας!