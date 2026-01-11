---
### Broadcast-ID'er

Du vil se, at du skal sende et `broadcastId` i nogle API-kald. Når du modtager hændelser, får du dette ID tilbage, så du ved, at du kan ignorere hændelsen, hvis du planlægger at anvende ændringer optimistisk på klienten (hvilket du sandsynligvis vil gøre, da det giver den bedste oplevelse). Pass et UUID her. ID'et bør være unikt nok til ikke at forekomme to gange i en session.

```swift
let broadcastId = UUID().uuidString
```
---