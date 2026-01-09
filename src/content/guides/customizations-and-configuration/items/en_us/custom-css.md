[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments is designed to be customized. The commenting widget itself runs inside an iframe for security reasons, so to apply
custom styling you have to follow one of two approaches.

The first, the easiest approach, and preferred by us, is to use the [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

In the widget customization page, see the "Show Advanced Options" section, under which there is an area labeled "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

This approach has some benefits:
1. The entered CSS is minified before it is sent to the user, and the formatting is kept consistent in the editing UI.
2. You get all the benefits of the widget customization UI, for example easily customizing the commenting widget differently for different sites.
3. When we make changes to the comment widget, your custom styling will be tested as part of our release process.

The second approach is to specify the **customCSS** parameter in the widget configuration, as follows:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

However, this has *limitations*:
1. There is a limit to how much custom CSS that can be passed before our servers will reject the request, due to the size of the headers.
2. You must manage the custom CSS in your infrastructure, and build system. This may be an upside rather than a downside, as well.
3. There is an additional overhead of sending the custom CSS over the network **twice** in this use case, as it has to be sent to our servers, and then sent back in the iframe content. However for most payload sizes, this is not noticeable.
4. A common optimization is minifying the CSS to reduce its size over the network, however with this approach you will have to handle that.
5. Your custom CSS won't be tested when we make changes.

### External CSS Files

You can tell the widget to fetch an external file by using `@import`!

It's recommended to put the `@import` in a customization rule. This way, if we ever need to make a change to the comment widget, we can use our automation
tooling to verify your setup. So for example, you would create a customization rule in the Widget Customization UI, click `Advanced`, and enter in `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

You can also load an external CSS file via the `customCSS` property:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

However, remember that your CSS won't be able to be tested by us if you do this. 

### User Profile Modal Styling

User profile modals can also be styled with custom CSS. However, to ensure that custom styling is applied to user profiles, all CSS selectors must be prefixed with `.user-profile`. Without this prefix, custom styling will be ignored for user profile modals.

For example:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

At FastComments, we know our customers customize the commenting widget. That's by design - the last thing we want is for our product to cause design
inconsistencies in your product.

Since this is an important part of our product, we have a build pipeline that allows us to review changes to the comment widget, per-customer, each release.

If we find minor issues, we will update your account to ensure our release goes smoothly. If we see major breaking changes, this allows us to halt the release.