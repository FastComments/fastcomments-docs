By default, the FastComments page size is `30`. This includes replies in threads.

The page size can be customized in the [Widget Configuration UI](https://fastcomments.com/auth/my-account/customize-widget) in varying sizes ranging from `10` to `200`.

Note that changing the page size requires recalculating all the comment threads in your account. This may take a couple of minutes.

This cannot be configured in the client side widget as pages are calculated server-side.

Example configuration is shown below:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.page-size'; title='Custom Page Sizes' app-screenshot-end]

Page sizes can be customized globally, or per-domain, or per-page, by creating different customization rules.

This will affect all clients, integrations, and frameworks that you may be using to show comments via our platform.
