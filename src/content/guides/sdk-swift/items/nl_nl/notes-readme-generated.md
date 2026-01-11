### Broadcast-ID's

Je zult zien dat je een `broadcastId` moet doorgeven in sommige API-aanroepen. Wanneer je gebeurtenissen ontvangt, krijg je dit ID terug, zodat je weet dat je de gebeurtenis kunt negeren als je van plan bent wijzigingen optimistisch op de client toe te passen (wat je waarschijnlijk wilt doen omdat het de beste gebruikerservaring biedt). Geef hier een UUID door. Het ID moet uniek genoeg zijn om niet twee keer in één sessie voor te komen.

```swift
let broadcastId = UUID().uuidString
```