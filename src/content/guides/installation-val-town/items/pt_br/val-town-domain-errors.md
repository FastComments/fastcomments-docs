Once you switch off the `demo` tenant, the widget may refuse to load with an authorization error. This is because FastComments doesn't know it's supposed to allow your account to be used on that domain.

[Go here to add your site to your account.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town is worth a second look here, because a val can be reachable at more than one hostname:

- Every HTTP val has a long default endpoint, `<org>--<id>.web.val.run`.
- Claiming a custom subdomain adds `<name>.val.run`.
- A [custom domain](https://docs.val.town/vals/http/custom-domains/) adds a third.
- Branches get their own URLs.

Add whichever hostnames you actually serve the widget from. If you claim a subdomain after setting things up, add that too, or the widget works on the old URL and fails on the new one.