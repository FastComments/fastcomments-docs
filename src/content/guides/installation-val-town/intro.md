[Val Town](https://val.town) runs TypeScript on Deno, so a val is a real server. That makes it a good fit for FastComments: the widget is a script tag on the page, and anything that needs a secret, like Secure SSO or verifying a webhook, can run server-side in the same val.

This guide covers adding the comment widget to an HTTP val, showing comment counts on an index page, signing users in with the Val Town account they already have, and receiving comment webhooks.

You don't need an account to try it. The examples use `tenantId: "demo"`, a shared sandbox, and Step 2 covers switching to your own.
