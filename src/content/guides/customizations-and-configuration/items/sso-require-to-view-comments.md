FastComments SSO (<a href="#sso">details here</a>) provides your users with a way to comment without having to log in to another platform.

However, this alone doesn't secure your comment threads, since by default comment data is publicly available information - anybody that can view the page can view the comments.

By changing a setting, we can restrict comments from being fetched unless it is by an administrator or valid SSO user.

#### No-Code Setup

We can prevent viewing and interacting with our comment threads, when SSO is set up, by creating a <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">customization rule</a>.

When doing so, search for SSO, and you will find this option:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Enable it and save the customization rule.

#### Only Protect a Certain Domain or Page

To only protect a certain Domain or Page, we'll simply configure the customization rule to do so.

At the top of the customization UI, we'll find two inputs, Domain and URL ID.

To just protect a particular domain, enter the domain in question into the "domain" field.

To protect a particular page, enter a page URL in the "URL ID" field. If you have a custom integration with FastComments, you may enter a type of ID here instead of a URL.

#### Protection Beyond Reading

Enabling this option will protect the page or domain from being commented on unless the user is logged in via SSO.

#### Gotchas

Any users that created comments before your SSO integration will not be able to see them, unless they log in via your SSO integration.
