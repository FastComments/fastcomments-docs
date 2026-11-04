### Broadcast Ids

Je zult zien dat je in sommige API-aanroepen een `broadcastId` moet meegeven. Wanneer je events ontvangt, krijg je deze ID terug, zodat je het event kunt negeren als je van plan bent om wijzigingen optimistisch toe te passen op de client
(waarschijnlijk wil je dat doen, omdat dit de beste ervaring biedt). Geef hier een UUID mee. De ID moet uniek genoeg zijn om niet twee keer voor te komen binnen een browsersessie.

### SSO (Single Sign-On)

Voor SSO-voorbeelden, zie hieronder.