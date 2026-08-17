[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

When a user comments with FastComments for the first time we will try fetch their avatar from <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

However, if we don't find an avatar, or the user never sets one in their account, we render a static default avatar image.

To specify your own static avatar image we can use the *defaultAvatarSrc* setting.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

This can also be done without code. In the widget customization page, see the "Default Avatar" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Default Avatar section of the widget customization page, where you set the fallback avatar image URL'; title='Customizing The Default Avatar' app-screenshot-end]

Note that defining the avatar for a particular user, like with SSO, is covered in its own section.
