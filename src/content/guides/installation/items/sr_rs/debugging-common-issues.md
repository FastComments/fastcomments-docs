Here are some symptoms we see encountered frequently, and common solutions. 

### "Ово је демо" Порука

This is shown when you've copied the widget code from our home page, which uses our demo
tenant. To use your tenant, copy the widget code from [овде](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Грешка

FastComments needs to know which domains are owned by you to authenticate requests associated
with your account. [Погледајте нашу документацију](/guide-multiple-sites.html#add-domains-to-account) to see how
to resolve this error (simply add the exact subdomain + domain to your account).

Note that this should only occur after the trial period is over. During the trial period, any requests from new domains
will automatically be added to your account.

### Пресетељени коментари се не приказују за прилагођене инсталације

Usually this happens when the imported comments are tied to a `Page ID`, and you are passing a URL
(or no value, in which case it defaults to the page URL).

You can debug this by [извести своје коментаре](https://fastcomments.com/auth/my-account/manage-data/export) and viewing the `URL ID` column (currently Column `B`).

Ensure the values you see in the `URL ID` column are the same values you are passing to the widget
configuration as the `urlId` parameter.

For further explanation, try reading our [Документација о везивању коментара за странице и чланке](/guide-customizations-and-configuration.html#url-id).

If all else fails, [контактирајте нас](https://fastcomments.com/auth/my-account/help).

### Виџет за коментаре се не приказује

If the comment widget isn't showing, check the Chrome developer console for errors.

For most misconfiguration, the comment widget will at least show an error on the page if it is
able to load. Seeing nothing is usually an indication of a scripting error.

### Жељена конфигурација не ради како се очекује

Try our [Chrome екстензија](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) to see what
configuration the comment widget is being passed. If all fails, take as screenshot of what the chrome extension says
and [контактирајте нас](https://fastcomments.com/auth/my-account/help).

### Коментари недостају на истом URL-у са различитим хеш‑бангом

By default, FastComments will use the page URL for the "bucket" where comments are stored. If your URLs include `#hashbangs`, and these `#hashbangs`
should not be part of the identifier that identifies a comment thread, we can simply ignore the hash bang value, for example:

[inline-code-attrs-start title = 'Пример игнорисања хеш‑банга'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Note that after making this change, a migration will have to be preformed for existing comments. [За то, контактирајте нас.](https://fastcomments.com/auth/my-account/help)

### Параметри упита у URL-у који утичу на виџет

By default, FastComments will use the page URL for the "bucket" where comments are stored. If your URLs include query parameters
that should not be part of the identifier that identifies a comment thread, we can simply ignore them, for example:

[inline-code-attrs-start title = 'Игнорисање параметара упита'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Note that after making this change, a migration will have to be preformed for existing comments. [За то, контактирајте нас.](https://fastcomments.com/auth/my-account/help)

### Не примате е‑мейлове

At FastComments, we put a lot of work into ensuring our delivery of emails is as reliable as
possible. However, some email providers are notoriously difficult to deliver to reliably. Check your spam
folder for messages from fastcomments.com.

If you [контактирате нас](https://fastcomments.com/auth/my-account/help) we can usually provide
more insight into why you may not be seeing emails from us.

---