### Broadcast Ids

Du vil se, at du skal sende en `broadcastId` i nogle API-kald. Når du modtager events, får du dette ID tilbage, så du ved, at du skal ignorere eventet, hvis du planlægger at anvende ændringer optimistisk på klienten
(sandsynligvis noget, du vil gøre, da det giver den bedste oplevelse). Send en UUID her. ID'et skal være unikt nok til ikke at forekomme to gange i en browsersession.

### SSO (Single Sign-On)

For SSO-eksempler, se nedenfor.