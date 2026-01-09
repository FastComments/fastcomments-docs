[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Es posible que observe que el widget de comentarios puede usarse con un Tenant ID de "demo", por ejemplo:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Esto está destinado únicamente a probar y jugar con el widget de comentarios. En producción, usted pasaría su Tenant ID, de la siguiente manera:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Puede encontrar su Tenant ID ya aplicado en el <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">fragmento de código</a> del widget de comentarios en su cuenta.

También puede encontrar su Tenant ID y gestionar sus claves de API [en la página de credenciales de API](https://fastcomments.com/auth/my-account/api-secret).

A partir de este punto, si ha iniciado sesión en FastComments, los ejemplos de código usarán su Tenant ID real (si ha iniciado sesión en https://fastcomments.com).