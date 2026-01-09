[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

We've covered how `urlId` is the page or article id the comments are tied to.

Also, to recap, if not defined the `urlId` will default to the current page URL.

What about SPAs, or Single-Page-Applications, where the page or content the comments are tied to changes
dynamically without a fresh page reload?

#### Angular, React, Vue, etc

With our libraries such as Angular and React, simply updating the `urlId` property passed to the widget
will cause the comment widget to refresh. You can see this in action for the React app, for example, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">burada</a>.

#### VanillaJS

If you're using the VanillaJS library it is slightly more complicated as there isn't a framework like Angular or React
to handle the data binding or state propagation.

When you instantiate the VanillaJS widget, it returns some functions which can be called to update it.

Here's a functional example where we change the page hash and update the comment widget:

[inline-code-attrs-start title = 'Sayfa Hash Değiştirme Örneği'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // We update url, too, so notifications can link back to the right page
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---