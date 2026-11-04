Το snippet FastComments για Live Comments στο Framer είναι παρακάτω.

[inline-code-attrs-start title = 'FastComments Framer-Specific Comments Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Εναλλακτικά, μπορείτε να χρησιμοποιήσετε το widget Streaming Chat. Το snippet FastComments Streaming Chat για το Framer είναι:

[inline-code-attrs-start title = 'FastComments Framer-Specific Streaming Chat Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Το FastComments υποστηρίζει τον επεξεργαστή Framer, οπότε θα πρέπει να δείτε κάτι τέτοιο αφού επικολλήσετε τον κώδικα (ίσως χρειαστεί να κάνετε κλικ στο `Publish`):

<div class="screenshot white-bg">
    <div class="title">Προεπισκόπηση Widget Σχολίων</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Προεπισκόπηση Widget Σχολίων" />
</div>

Τώρα, όταν προβάλλετε τον ιστότοπό σας, θα πρέπει να βλέπετε την περιοχή σχολίων! Στην πλαϊνή γραμμή του Framer μπορείτε, αν θέλετε, να ορίσετε το widget και σε πλήρες πλάτος.

Σημειώστε ότι το Framer περιορίζει το ύψος των widgets και δεν υποστηρίζει αυτόματη προσαρμογή μεγέθους, γι' αυτό έχουμε επιλέξει εδώ το widget Live Chat, καθώς έχει σταθερό ύψος.