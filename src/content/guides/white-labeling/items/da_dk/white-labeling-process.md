---
Processen for opsætning af White Labeling er som følger:

1. Vælg, hvordan fakturering skal håndteres.
   1. Med FastComments Pro betaler du et fast beløb for op til et bestemt antal white-labeled tenants.
   2. Med FastComments Flex betaler du for hver tenant og for brugen af den pågældende tenant.
   3. Du sætter grænserne for hver tenant i begge tilfælde.
      1. Grænser kan tilpasses pr. tenant. Derudover, hvis du opdaterer de pakker, du sælger, kan du gøre det uden at forstyrre den prisfastsættelse, du allerede har givet til eksisterende kunder.
2. Gør dig fortrolig med FastComments.com-terminologien:
   1. `Tenant` er en "kunde".
   2. `TenantUser` er en administrator med et sæt privilegier i `Tenant`.
   3. `TenantPackage` er en pakke med et sæt grænser og priser tilgængelige for en `Tenant`.
3. Integrer med vores [API](/guide-api.html) eller brug [skripter](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling) til at onboarde tenants.

---