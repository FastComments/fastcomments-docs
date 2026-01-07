In this next step you need to copy the pre-made widget code below.

As long as you're logged into FastComments.com the below code snippet will already have your account information. Let's copy it:

[inline-code-attrs-start title = 'Super.so FastComments Collab Chat Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Clean up existing instance
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Clean up existing top bar if it exists
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Create new top bar
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Initialize FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Update current pathname
            currentPathname = window.location.pathname;
        }

        // Initial load
        load();

        // Check every 500ms for changes
        setInterval(() => {
            // Reload if pathname changed
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Reload if widget was removed
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Reload if container was emptied
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Now paste in the `Body` area:

<div class="screenshot white-bg">
    <div class="title">Pasted Code</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Pasted Code" />
</div>

If you see a "this is a demo message" after pasting the code:

- Ensure you're logged into your fastcomments.com account.
- Ensure you have 3rd party cookies enabled.
- Then refresh this page and copy the code snippet again. It should have `tenantId` populated with your tenant's identifier.
