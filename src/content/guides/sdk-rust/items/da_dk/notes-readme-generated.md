### Broadcast IDs

Du vil se, at du skal sende en `broadcastId` i nogle API-opkald. Når du modtager events, får du denne ID tilbage, så du kan ignorere eventet, hvis du planlægger at anvende ændringer optimistisk på klienten
(hvilket du sandsynligvis vil gøre, da det giver den bedste oplevelse). Send en UUID her. ID'en bør være unik nok til ikke at forekomme to gange i en browsersession.