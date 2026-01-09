Det er almindeligt at have en sub-tenant per test- eller dev-miljø med FastComments. Hver tenant vil have sin egen konfiguration, data og API-nøgler. Konfiguration, data og brugere kan ikke deles på tværs af tenants.
Alt er isoleret. Dog kan superadmins af parent-tenanten udgive sig for brugere i child-tenants.

There are two approaches:

- Den primære tenant bruges til produktion, og sub-tenants bruges til testmiljøer.
- Den primære tenant bruges simpelthen til fakturering, og hver sub-tenant er til produktion, test osv.

Den første er generelt lettere for brugere at forholde sig til, men det kan afhænge af din organisation.

Tenants kan oprettes [her](https://eu.fastcomments.com/auth/my-account/tenants) hvis du har pakken. Det er også her, superadmins ville udgive sig for brugere. Tenants kan også oprettes via API'et til mere tilpassede/automatiserede opsætninger.

Uanset hvilken tilgang der vælges, skal du tilføje moderatorerne og brugerne, som ønsker at se produktionsdata, i "prod"-tenanten. Så for eksempel, hvis du vælger mulighed B og har parent-tenanten til fakturering, og en sub-tenant til "prod", skal du tilføje tenant'en, skifte til den nye tenant og tilføje dine admin- og moderatorbrugere for sub-tenanten. 

Endelig, for at præcisere: siden "Moderate Comments" vil være tom med mulighed B for parent-tenanten.