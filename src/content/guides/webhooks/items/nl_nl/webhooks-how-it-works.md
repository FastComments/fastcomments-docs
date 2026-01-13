Alle wijzigingen aan het Comment object in het systeem veroorzaken een gebeurtenis die op een wachtrij terechtkomt.

Het initiële webhook-evenement wordt meestal binnen zes seconden na het optreden van de gebeurtenisbron verzonden.

U kunt deze wachtrij in de Webhooks-admin monitoren voor het geval uw API uitvalt.

Als een verzoek naar uw API faalt, plaatsen we het opnieuw in de wachtrij volgens een schema.

Dat schema is `1 Minute * the retry count`. Als de oproep één keer faalt, zal er opnieuw een poging worden gedaan na
een minuut. Als het twee keer faalt, zal het vervolgens twee minuten wachten, en zo verder. Dit is zodat we
uw API niet overbelasten als uw API uitvalt door load-gerelateerde redenen.

Webhooks kunnen worden geannuleerd vanaf de [logpagina](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).