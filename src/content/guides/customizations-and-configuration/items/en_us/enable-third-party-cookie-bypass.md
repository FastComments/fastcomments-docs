[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

For authentication, FastComments depends on third-party cookies being enabled in your browser. Without them, users will always have to provide their email to comment (unless the email input field is hidden), and their comments will always appear as unverified (by default).

To work around this, you can enable the third-party cookie bypass. 

When this setting is enabled, it causes a small popup that displays a message indicating the user is being logged in. This popup appears whenever the user interacts with the comment widget; for example, when they leave a comment.

We can do this in code by setting the **enableThirdPartyCookieBypass** flag to true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

We can also set this up via the Widget Customization UI, under `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]