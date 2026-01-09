Standaard synchroniseert FastComments dagelijks terug naar uw WordPress-site. Dit is uitsluitend voor back-updoeleinden zodat u een kopie van de gegevens blijft bezitten, en voor plugins die ervan afhankelijk kunnen zijn.

Dit gebeurt niet onmiddellijk bij elke opgeslagen reactie omdat sommige sites zware leesbelasting aankunnen, terwijl hun database-implementaties niet altijd zware schrijfbelasting aankunnen (vandaar dat dit werk wordt uitbesteed aan FastComments).

Het synchronisatieschema terug naar WordPress kan worden aangepast door een plugin te installeren. We raden [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description) aan.

Stappen:

1. Installeer WP Crontrol
2. Ga naar `Settings -> Cron Schedules`.
3. Ga naar het tabblad `Cron Events`.
4. Zoek naar `fastcomments_cron_hook`.
5. Bewerk het event. U kunt de hook configureren om elk uur, twee keer per dag, dagelijks (standaard), of eenmaal per week te laten draaien.

Het synchroniseren terug naar WordPress kan ook op elk gewenst moment handmatig worden uitgevoerd door naar het dashboard van de FastComments-plugin te gaan en `Manually Sync` te selecteren. U krijgt de optie om terug te synchroniseren naar uw WP-installatie, of uw WP-reacties opnieuw te uploaden naar de servers van FastComments.