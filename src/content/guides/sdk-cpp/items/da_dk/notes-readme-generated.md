### Broadcast-id'er

Du vil se, at du skal sende en `broadcastId` i nogle API-opkald. Når du modtager events, får du dette ID tilbage, så du kan ignorere eventet, hvis du planlægger at anvende ændringer optimistisk på klienten (hvilket du sandsynligvis vil gøre, da det giver den bedste oplevelse). Send en UUID her. ID'et bør være unikt nok til ikke at forekomme to gange i en browser-session.

### SSO (Single Sign-On)

For SSO-eksempler, se nedenfor.