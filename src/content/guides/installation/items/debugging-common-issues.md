Here are some symptoms we see encountered frequently, and common solutions. 

### "This is a demo" Message

This is shown when you've copied the widget code from our home page, which uses our demo
tenant. To use your tenant, copy the widget code from [here](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Error

FastComments needs to know which domains are owned by you to authenticate requests associated
with your account. [Check out our documentation](/guide-multiple-sites.html#add-domains-to-account) to see how
to resolve this error (simply add the exact subdomain + domain to your account).

Note that this should only occur after the trial period is over. During the trial period, any requests from new domains
will automatically be added to your account.

### Migrated Comments Not Showing for Custom Installations

Usually this happens when the imported comments are tied to a `Page ID`, and you are passing a URL
(or no value, in which case it defaults to the page URL).

You can debug this by [exporting your comments](https://fastcomments.com/auth/my-account/manage-data/export) and viewing the `URL ID` column (currently Column `B`).

Ensure the values you see in the `URL ID` column are the same values you are passing to the widget
configuration as the `urlId` parameter.

For further explanation, try reading our [How Comments are Tied to Pages and Articles documentation](/guide-customizations-and-configuration.html#url-id).

If all else fails, [reach out to us](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

If the comment widget isn't showing, check the Chrome developer console for errors.

For most misconfiguration, the comment widget will at least show an error on the page if it is
able to load. Seeing nothing is usually an indication of a scripting error.

### Desired Configuration Not Working as Expected

Try our [Chrome extension](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) to see what
configuration the comment widget is being passed. If all fails, take as screenshot of what the chrome extension says
and [reach out to us](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

By default, FastComments will use the page URL for the "bucket" where comments are stored. If your URLs include `#hashbangs`, and these `#hashbangs`
should not be part of the identifier that identifies a comment thread, we can simply ignore the hash bang value, for example:

[inline-code-attrs-start title = 'Ignore Hash Bangs Example'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Note that after making this change, a migration will have to be preformed for existing comments. [For that, reach out to us.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

By default, FastComments will use the page URL for the "bucket" where comments are stored. If your URLs include query parameters
that should not be part of the identifier that identifies a comment thread, we can simply ignore them, for example:

[inline-code-attrs-start title = 'Ignore Query Parameters'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Note that after making this change, a migration will have to be preformed for existing comments. [For that, reach out to us.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

At FastComments, we put a lot of work into ensuring our delivery of emails is as reliable as
possible. However, some email providers are notoriously difficult to deliver to reliably. Check your spam
folder for messages from fastcomments.com.

If you [reach out to us](https://fastcomments.com/auth/my-account/help) we can usually provide
more insight into why you may not be seeing emails from us.
