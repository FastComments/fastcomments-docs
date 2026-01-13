Het is gebruikelijk om per test- of ontwikkelomgeving een sub-tenant te hebben bij FastComments. Elke tenant heeft zijn eigen configuratie, gegevens en API-sleutels. Configuratie, gegevens en gebruikers kunnen niet tussen tenants worden gedeeld.
Alles is geïsoleerd. Superadmins van de ouder-tenant kunnen zich echter voordoen als gebruikers in sub-tenants.

Er zijn twee benaderingen:

- De hoofd-tenant is voor productie, en sub-tenants zijn voor testomgevingen.
- De hoofd-tenant is simpelweg voor facturering, en elke sub-tenant is voor prod, test, enzovoort.

De eerste optie is over het algemeen makkelijker voor gebruikers om te begrijpen, maar dit kan afhangen van uw organisatie.

Tenants kunnen [hier](https://eu.fastcomments.com/auth/my-account/tenants) worden aangemaakt als u het pakket heeft. Dit is ook waar superadmins zouden
zich voordoen als gebruikers. Tenants kunnen ook via de API worden aangemaakt voor meer aangepaste/geautomatiseerde configuraties.

Ongeacht de gekozen aanpak moet u de moderators en gebruikers toevoegen die productiedata willen zien in de "prod"-tenant. Dus bijvoorbeeld als u optie B wilt
kiezen en de ouder-tenant voor facturering wilt gebruiken, en een sub-tenant voor "prod" wilt hebben, wilt u de tenant toevoegen, overschakelen naar de nieuwe tenant, en uw
admin- en moderatorgebruikers voor de sub-tenant toevoegen. 

Ten slotte, ter verduidelijking, zal de pagina "Moderate Comments" leeg zijn bij optie B voor de ouder-tenant.