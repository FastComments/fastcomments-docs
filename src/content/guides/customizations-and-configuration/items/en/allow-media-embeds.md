By default FastComments does not allow iframes in comments. When you enable media embeds, commenters can paste the embed code (the `<iframe>` snippet) from trusted providers like YouTube, Vimeo, SoundCloud, and Spotify, and it will render inline in the comment.

For security, this is not a client-side widget config flag. It is a server-side setting, validated when each comment is saved, so it cannot be turned on from the page. Only iframes pointing at a built-in list of trusted providers are allowed. Any other iframe is removed.

This is done without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Adding Your Own Providers

If you want to allow embeds from a provider that is not on the built-in trusted list, add its hostname in the "Additional Embed Domains" field on the same page. These hostnames are allowed in addition to the built-in providers. Matching is exact, so include the full hostname (for example, player.example.com). Anything you do not list stays blocked.

Both the plain comment box and the WYSIWYG editor support pasting an embed. In the WYSIWYG editor the embed is inserted as a removable block.
