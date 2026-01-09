Comments themselves are not tied to a particular domain.

They are tied to your account, and the <a href="/guide-customizations-and-configuration.html#url-id" target="_blank">urlId</a> provided.

This means that we can render the same comment thread on two separate sites with the same FastComments account by simply passing in the same value for `urlId` in
the widget configuration.

In this scenario, comments posted on page A would immediately appear on page B, and vice-versa. Live commenting
also works as expected.

[You can find an example in this blog post](https://blog.fastcomments.com/(2-12-2020)-using-fastcomments-in-multiple-places-on-the-same-page.html).

---