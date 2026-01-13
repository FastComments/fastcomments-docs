By default, FastComments allows linking to any external site.

This can instead be restricted to a desired list of sites or domains. Attempting to post a link to a site or domain,
not in the defined list will cause an error to be shown to the user.

This validation is only for the Comment Widget and API. Imports are not affected.

This is done without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; title='Restrict External Link Domains' app-screenshot-end]