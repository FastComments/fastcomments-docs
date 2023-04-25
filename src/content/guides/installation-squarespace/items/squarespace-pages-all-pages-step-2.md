Now we can copy the following code snippet. Use the copy button that appears in the top right of the snippet.

[inline-code-attrs-start title = 'Squarespace All Pages Comments Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        function tryLoad() {
            const pageContainer = document.querySelector('#page.container');
            if (!pageContainer) {
                console.warn('FastComments Error - Page container not found, trying again.');
                setTimeout(tryLoad, 500);
                return;
            }
            const pageSection = document.createElement('section');
            pageSection.classList.add('page-section', 'content-width-wide');
            const contentWrapper = document.createElement('div');
            contentWrapper.classList.add('content-wrapper');
            const content = document.createElement('div');
            content.classList.add('content');
            const targetDiv = document.createElement('div');
            pageContainer.appendChild(pageSection);
            pageSection.appendChild(contentWrapper);
            contentWrapper.appendChild(content);
            content.appendChild(targetDiv);
            window.FastCommentsUI(targetDiv, {
                "tenantId": "demo"
            });
        }
        tryLoad();
    })();
</script>

[inline-code-end]

...then paste in the code area and click save. It should look like this, with the code in the `FOOTER` block:

<div class="screenshot white-bg">
    <div class="title">Paste and Save</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Paste and Save" />
</div>

If you're having issues, make sure near the bottom it doesn't say `"tenantId": "demo"`. It should show your tenant id if you are logged in. If not, reach out to support.
