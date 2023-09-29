In this next step you need to copy the pre-made widget code below.

As long as you're logged into FastComments.com the below code snippet will already have your account information. Let's copy it:

[inline-code-attrs-start title = 'Super.so FastComments Collab Chat Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }
            const target = document.querySelector('.super-content');
            if (!target) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget
            });
        }

        load();
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
