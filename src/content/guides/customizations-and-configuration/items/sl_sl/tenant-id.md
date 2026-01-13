[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Opazili boste, da je mogoče pripomoček za komentarje uporabiti s Tenant ID-jem "demo", na primer:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

To je namenjeno le preizkušanju in igranju s pripomočkom za komentarje. V produkciji boste posredovali svoj Tenant ID, na primer:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Svoj Tenant ID lahko najdete že uporabljen v <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">izrezku kode za pripomoček za komentarje v vašem računu</a>.

Prav tako lahko svoj Tenant ID najdete in upravljate svoje API ključe [na strani z API poverilnicami](https://fastcomments.com/auth/my-account/api-secret).

Od tega trenutka naprej, če ste prijavljeni v FastComments, bodo primeri kode uporabljali vaš pravi Tenant ID (če ste prijavljeni na https://fastcomments.com).