Wat er op je kaart wordt afgeschreven, en wanneer, hangt af van het soort abonnement dat je hebt.

### Vaste abonnementen

Wanneer je een `Creators`- of `Pro`-abonnement start, wordt de eerste maand of het eerste jaar direct afgerekend, in de browser, wanneer je op `Pay Now & Start Subscription` klikt. Als je bank je vraagt de betaling te bevestigen, voltooi dan de controle in het venster dat opent. Het abonnement start pas zodra de betaling is afgerond.

Daarna wordt het abonnement vernieuwd op dezelfde dag van elke maand, of van elk jaar als je voor `Pay Annually` hebt gekozen, gerekend vanaf de dag waarop je je hebt aangemeld. De vernieuwing wordt automatisch afgeschreven van de kaart die op bestand staat en de factuur wordt gemaild naar je facturatie-e-mailadres. Er is geen actie nodig.

Jaarlijkse facturering is 15% goedkoper dan maandelijks. Flex kan niet jaarlijks worden afgerekend.

### Flex

Flex wordt achteraf afgerekend: je gebruikt de dienst tijdens een kalendermaand, en de factuur voor die maand wordt op de laatste dag van de maand (UTC) aangemaakt en direct van je kaart afgeschreven. De factuurdatum is altijd de laatste dag van de maand en kan niet worden gewijzigd.

De factuur heeft alleen betrekking op die kalendermaand. Gebruik in de kalendermaand waarin je je hebt aangemeld wordt nooit gefactureerd, dus je eerste factuur is vroegst aan het einde van de maand na je aanmelding.

Als je gebruik onder het Flex-minimum blijft, zorgt een regel met `FastComments Flex. (minimum ... /mo)` ervoor dat de factuur wordt aangevuld tot het minimumbedrag. Het huidige minimum wordt getoond op je [Billing Info](https://fastcomments.com/auth/my-account/billing-info)-pagina en op de [prijspagina](https://fastcomments.com/traffic-pricing).

Je kunt op elk moment zien wat de volgende factuur zal zijn. Het `Next Invoice Amount` staat op de [Billing Info](https://fastcomments.com/auth/my-account/billing-info)-pagina, en de volledige uitsplitsing regel voor regel staat op de [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing)-pagina. Wat er wordt geteld, staat beschreven in [How Flex Usage Is Measured](#how-flex-usage-is-measured).

### Een kaart opslaan leidt niet tot afschrijving

Zodra je facturering is ingesteld, wordt het opslaan of bijwerken van een kaart zonder wijziging van abonnement nooit afgeschreven, tenzij je een onbetaalde factuur hebt. Het formulier zegt `You won't be charged now unless you change plans.` en de kaart wordt simpelweg bij Stripe opgeslagen voor de volgende factuur. Alleen als je nog geen geldige facturering hebt, zegt het formulier `Your card will be charged now.`, en zelfs dan wordt een Flex-account pas afgeschreven aan het einde van de maand. Eventueel voorbetaald [tegoed](#credits) op je account wordt eerst op elke factuur toegepast voordat de kaart wordt afgeschreven.

### Beheerde sub-accounts

Als je account sub-accounts beheert (bijvoorbeeld één per site of omgeving, of white-label tenants), wordt alleen het hoofdaccount gefactureerd. Het gebruik van elk sub-account verschijnt als eigen regels op de Flex-factuur van het hoofdaccount, voorzien van de naam van het sub-account, en sub-accounts delen de betaalmethode en factureringsstatus van het hoofdaccount.