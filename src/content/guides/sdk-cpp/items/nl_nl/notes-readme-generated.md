### Broadcast-id's

Je zult zien dat je in sommige API-aanroepen een `broadcastId` moet meegeven. Wanneer je gebeurtenissen ontvangt, krijg je dit ID terug, zodat je het evenement kunt negeren als je van plan bent wijzigingen optimistisch aan de client toe te passen
(wat je waarschijnlijk wilt doen omdat het de beste ervaring biedt). Geef hier een UUID door. Het ID moet uniek genoeg zijn om niet twee keer in een browsersessie voor te komen.

### SSO (Single Sign-On)

Zie hieronder voor SSO-voorbeelden.