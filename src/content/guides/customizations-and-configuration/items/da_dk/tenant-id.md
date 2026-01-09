[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Du vil måske bemærke, at kommentar-widgeten kan bruges med en Tenant ID på "demo", for eksempel:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Dette er kun ment til at prøve og lege med kommentar-widgeten. I produktion ville du angive din Tenant ID, således:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Din Tenant ID kan findes allerede indsat i kommentar-widgetens <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">kodeeksempel i din konto</a>.

Du kan også finde din Tenant ID og administrere dine API-nøgler [på siden med API-legitimationsoplysninger](https://fastcomments.com/auth/my-account/api-secret).

Fremover, hvis du er logget ind på FastComments, vil kodeeksemplerne bruge din rigtige Tenant ID (hvis du er logget ind på https://fastcomments.com).