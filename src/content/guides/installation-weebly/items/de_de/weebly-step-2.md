---
Um die Integration von Weebly und FastComments reibungslos zu gestalten, müssen wir **zwei** kleine Code-Snippets hinzufügen.

Das erste Snippet soll die Weebly-Nachricht "Comments are Closed" ausblenden, das zweite lädt tatsächlich FastComments.

Kopiere zuerst dieses kleine Code-Snippet:

[inline-code-attrs-start title = 'FastComments Kopfzeilen-Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Dann klicke auf derselben Einstellungsseite aus `Step One` auf das `+` neben `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Post-Header-Code öffnen</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Post-Header-Code öffnen" />
</div>

Ein Textfeld sollte sich wie folgt öffnen:

<div class="screenshot white-bg">
    <div class="title">Post-Header-Code geöffnet</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Post-Header-Code geöffnet" />
</div>

Füge nun unser Code-Snippet ein:

<div class="screenshot white-bg">
    <div class="title">Header-Code-Snippet eingefügt</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Header-Code-Snippet eingefügt" />
</div>

Als Nächstes kommt der Footer-Code, um FastComments zu aktivieren. Klicke auf das Pluszeichen neben `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Post-Footer-Code öffnen</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Post-Footer-Code öffnen" />
</div>

Kopiere dieses Code-Snippet, das **speziell für Weebly** entwickelt wurde:

[inline-code-attrs-start title = 'FastComments Fußzeilen-Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // Schaltfläche 'Kommentare anzeigen' entfernen
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

Füge nun unseren Footer-Code ein:

<div class="screenshot white-bg">
    <div class="title">Post-Footer-Code hinzugefügt</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Post-Footer-Code hinzugefügt" />
</div>

Das war's!

---