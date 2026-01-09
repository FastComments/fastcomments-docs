### Broadcast-id'er

Du vil se, at du skal sende en `broadcastId` i nogle API-kald. Når du modtager begivenheder, får du dette ID tilbage, så du kan ignorere begivenheden, hvis du planlægger at anvende ændringer optimistisk på klienten
(hvilket du sandsynligvis vil gøre, da det giver den bedste oplevelse). Angiv en UUID her. ID'en skal være unik nok til ikke at forekomme to gange i samme browser-session.