Een betaling kan mislukken omdat een kaart is verlopen, is vervangen, geen saldo heeft, of omdat de bank de afschrijving heeft geblokkeerd. FastComments geeft je tijd om dit op te lossen voordat er iets stopt met werken, en informeert de mensen die het kunnen oplossen.

### Wat Er Gebeurt

1. **De afschrijving mislukt.** De factuur blijft `Unpaid` in de [Betalingshistorie](https://fastcomments.com/auth/my-account/billing-history) met een `Pay Now`-knop. Bij een vast plan kan Stripe de afschrijving opnieuw proberen. Bij Flex probeert FastComments elke nacht opnieuw facturen die ouder zijn dan de respijtperiode en nog niet betaald zijn.
2. **Er wordt een waarschuwing verstuurd.** Elke nacht controleert FastComments elk account. Als een Flex-factuur na de respijtperiode nog steeds onbetaald is en de nieuwe poging mislukt, of als een abonnement op een vast plan is beëindigd omdat het niet kon worden geïncasseerd, ontvangt elke gebruiker met de `Billing Admin`-machtiging de e-mail `Your FastComments Tenant Will Be Deactivated Soon!` met de datum, en wordt het account gepland voor deactivering één respijtperiode later. De widget blijft in de tussentijd gewoon werken.
3. **Deactivering.** Als er op de geplande datum niets is veranderd, wordt het account gedeactiveerd: reacties worden verborgen op je site en bezoekers zien `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Factureringsbeheerders ontvangen opnieuw een e-mail. Je gegevens worden bewaard.

De respijtperiode is één week voor de meeste accounts en twee weken voor accounts ouder dan een jaar. Bij Flex geldt de periode twee keer, eenmaal vóór de waarschuwing en eenmaal tussen de waarschuwing en de deactivering, zodat er minstens twee weken zitten tussen een mislukte afschrijving en het verbergen van reacties.

### Het Oplossen

Sla een werkende kaart op op de pagina [Factureringsgegevens](https://fastcomments.com/auth/my-account/billing-info). Dat alleen activeert het account onmiddellijk opnieuw en annuleert elke geplande deactivering; de onbetaalde factuur wordt vervolgens geïncasseerd met de nieuwe kaart, onmiddellijk bij een vast plan en tijdens de nachtelijke controle bij Flex. Je kunt de factuur ook meteen betalen met `Pay Now` in de Betalingshistorie.

Als de kaart in ons bestand in orde is en de mislukking tijdelijk was, hoef je niets te doen: de nachtelijke controle merkt de betaalde factuur of het actieve abonnement op, annuleert de deactivering en stuurt de factureringsbeheerders een bevestigings-e-mail. Een reeds gedeactiveerd account wordt op dezelfde manier opnieuw geactiveerd.

### Meerdere Facturen Tegelijk

Als je kaart een paar maanden ongeldig was en daarna is hersteld, wordt elke onbetaalde maand apart geïncasseerd, dus je kunt meerdere afschrijvingen op dezelfde dag zien. Controleer de factureringsperiode op elke factuur in de Betalingshistorie: ze zijn voor verschillende maanden, geen duplicaten.

### Nog Steeds Een Factureringswaarschuwing Zien Na Betaling?

De waarschuwing hoort bij een specifiek account. Als je hebt betaald maar een site toont nog steeds `billing information is out of date`, gebruikt de widget op die site hoogstwaarschijnlijk een ander account dan het account waarop je hebt betaald. Dit gebeurt wanneer een site twee keer is opgezet, of wanneer er één account is op `fastcomments.com` en een ander op de EU-implementatie op `eu.fastcomments.com`, die apart worden gefactureerd. Vergelijk de `tenantId` in de code op je pagina met die onder `Settings -> API/SSO`, of [neem contact op met de support](https://fastcomments.com/auth/my-account/help) en we lossen het voor je op.