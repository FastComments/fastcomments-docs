### Broadcast-id'er

Du vil se, at du skal sende en `broadcastId` i nogle API-kald. Når du modtager begivenheder, får du dette ID tilbage, så du ved, at du skal ignorere begivenheden, hvis du planlægger at anvende ændringer optimistisk på klienten (hvilket du sandsynligvis vil gøre, da det giver den bedste oplevelse). Send en UUID her. ID'et bør være unikt nok til ikke at forekomme to gange i en browsersession.

### SSO (Single Sign-On)

For SSO-eksempler, se nedenfor.