### Broadcast Ids

Du vil se, at du skal sende en `broadcastId` i nogle API-kald. Når du modtager events, får du dette ID tilbage, så du ved, at du kan ignorere eventet, hvis du planlægger at anvende ændringer optimistisk på klienten
(hvilket du sandsynligvis vil, da det giver den bedste oplevelse). Send en UUID her. ID'et skal være unikt nok til ikke at optræde to gange i en browsersession.

### SSO (Single Sign-On)

For SSO-eksempler, se nedenfor.