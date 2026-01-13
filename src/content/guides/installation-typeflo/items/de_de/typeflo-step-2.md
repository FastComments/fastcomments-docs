Fügen Sie im Bereich **Footer** der Registerkarte 'Custom Code' den folgenden Code ein:

[inline-code-attrs-start title = 'Typeflo.io Live-Kommentare Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Code im Footer-Bereich einfügen</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="FastComments-Code in den Footer-Bereich einfügen" />
</div>

Nachdem Sie den Code eingefügt haben, klicken Sie auf die **Save**-Schaltfläche, um Ihre Änderungen zu übernehmen.

Hinweis: Dieser Code enthält Logik, um das FastComments-Widget dynamisch an der optimalen Stelle in Ihren Typeflo.io-Blogbeiträgen zu platzieren. Andere Code-Snippets funktionieren nicht korrekt mit dem Layout von Typeflo.io.

Denken Sie daran, 'demo' durch Ihre tatsächliche FastComments-Tenant-ID zu ersetzen, nachdem Sie sich registriert haben. Wenn Sie bei FastComments.com angemeldet sind, sollte es bereits ersetzt sein.