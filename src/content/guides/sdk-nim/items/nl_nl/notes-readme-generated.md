### Broadcast Ids

Je zult zien dat je in sommige API-aanroepen een `broadcastId` moet doorgeven. Wanneer je events ontvangt, krijg je deze ID terug, zodat je weet dat je het event moet negeren als je van plan bent om wijzigingen optimistisch toe te passen op de client
(waarschijnlijk wil je dat doen, omdat dit de beste ervaring biedt). Geef hier een UUID door. De ID moet uniek genoeg zijn om niet twee keer voor te komen in een browsersessie.

### SSO (Single Sign-On)

Voor SSO-voorbeelden, zie hieronder.