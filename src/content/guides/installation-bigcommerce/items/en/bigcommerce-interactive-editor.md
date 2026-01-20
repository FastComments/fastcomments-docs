It's not recommended adding FastComments via BigCommerce's Page Builder as then the code has to be manually added to every desired page.

However, if this is desired, the following code snippet must be used. Code snippets from other tutorials will not work due to the nature of BigCommerce:

[inline-code-attrs-start title = 'BigCommerce Page Builder Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]
