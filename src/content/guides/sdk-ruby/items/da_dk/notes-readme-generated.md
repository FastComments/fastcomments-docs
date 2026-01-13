### Broadcast-id'er

Du vil se, at du skal sende et `broadcastId` i nogle API-opkald. Når du modtager hændelser, får du dette ID tilbage, så du ved, at du skal ignorere hændelsen, hvis du har tænkt dig at anvende ændringer optimistisk på klienten
(hvilket du sandsynligvis vil gøre, da det giver den bedste oplevelse). Send en UUID her. ID'et bør være tilstrækkeligt unikt til ikke at forekomme to gange i en browsersession.

### SSO (Single Sign-On)

For SSO-eksempler, se nedenfor.