### Broadcast-id's

Je zult zien dat je in sommige API-aanroepen een `broadcastId` moet meegeven. Wanneer je gebeurtenissen ontvangt, krijg je deze ID terug, zodat je weet dat je de gebeurtenis moet negeren als je van plan bent wijzigingen optimistisch op de client toe te passen
(wat je waarschijnlijk wilt doen omdat het de beste ervaring biedt). Geef hier een UUID door. De ID moet voldoende uniek zijn zodat deze niet twee keer in een browsersessie voorkomt.

### SSO (Single Sign-On)

Voor SSO-voorbeelden, zie hieronder.