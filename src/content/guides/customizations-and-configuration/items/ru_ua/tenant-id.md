[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

You may notice that the comment widget can be used with a Tenant ID of "demo", for example:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

This is only meant to try out and play with the comment widget. In production, you would pass your Tenant ID, like so:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Your Tenant ID can be found already applied on the comment widget <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">фрагменті коду у вашому акаунті</a>.

You can also find your Tenant ID and manage your API keys [на сторінці облікових даних API](https://fastcomments.com/auth/my-account/api-secret).

From this point on, if you are logged into FastComments, the code examples will use your real Tenant ID (if you are logged in on https://fastcomments.com).

---