Elke afschrijving, of het nu gaat om een planverlenging, een Flex-maand of een planwijziging, produceert een factuur. Facturen dienen tegelijk als ontvangstbewijs zodra ze betaald zijn.

### Facturen Vinden

Open [Billing History](https://fastcomments.com/auth/my-account/billing-history), of klik op `View Invoices` op de Billing Info-pagina. De tabel toont elke factuur met de datum, factuurperiode, factuurnummer, omschrijving, totaal, status (`Paid`, `Unpaid`, `Draft` of `Void`) en vervaldatum, tien per pagina, nieuwste eerst.

Voor elke factuur:

- `View` opent de factuur op de gehoste pagina van Stripe, waar je deze ook kunt downloaden.
- `PDF` downloadt de PDF rechtstreeks.
- `Pay Now` verschijnt in plaats van `View` bij een onbetaalde factuur en brengt je naar de betalingspagina van Stripe voor die factuur, waar je deze direct kunt betalen.

Facturen worden ook gemaild naar je facturatie-e-mailadres zodra ze worden aangemaakt. Laat ze naar een financieel postvak gaan in plaats van naar de accounteigenaar door de `Billing Email` in te stellen in [Invoice Settings](#invoice-settings).

### Wat Er Op De Factuur Staat

De factuur toont de factuurnaam en het adres en het VAT / Tax ID uit [Invoice Settings](#invoice-settings), de factuurregels, eventueel toegepast voorbetaald [credit](#credits), en het bedrag dat aan je kaart is afgeschreven. Flex-facturen vermelden één regel per soort gebruik, zie [How Flex Usage Is Measured](#how-flex-usage-is-measured). Voor factuuradressen in de Europese Unie worden Flex-facturen uitgegeven met 0% BTW en de voettekst `Reverse Charge Transaction: Customer to pay VAT`.

Wijzigingen in factuurinstellingen gelden voor facturen die worden aangemaakt nadat je ze hebt opgeslagen. Een reeds uitgegeven factuur kan niet worden bewerkt, dus stel je bedrijfsgegevens en VAT / Tax ID in vóór de volgende factuurdatum.

### Accounts Die Elders Worden Gefactureerd

Als je facturering extern wordt afgehandeld, bijvoorbeeld via een factuurafspraak met support, worden facturen naar je verstuurd voor betaling in plaats van aan een kaart afgeschreven en verschijnen ze mogelijk niet in Billing History. Accounts die via Shopify of BigCommerce zijn geïnstalleerd, worden door dat platform gefactureerd en de Billing Info-pagina linkt naar de planpagina daar. Sub-accounts die via een hoofdaccount worden gefactureerd, hebben geen eigen facturen; zie [When You Are Charged](#when-you-are-charged).