The smallest extension possible would be:

[inline-code-attrs-start title = 'A Simple Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

For the sake of this example, save this as `my-extension.js`, and make it available at `https://example.com/my-extension.min.js`.

This extension does not do anything, except on load it fetches the extension object created by the core comment library.

This `Extension` object is a singleton and is not shared with any other extensions.

Next, to load our extension, we have to tell the comment widget about it. For example:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

For functional examples, see the next section.
