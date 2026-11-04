## Forbind din konto

1. I Zapier skal du tilføje et FastComments-trin til en Zap eller åbne FastComments-appsiden i Zapier App Directory.
2. Vælg **Sign in to FastComments**. Zapier spørger først til din region: vælg **United States**, medmindre din
   konto blev oprettet i EU-regionen (`eu.fastcomments.com`).
3. Et FastComments-vindue åbnes. Log ind, hvis du ikke allerede er logget ind.
4. Gennemgå samtykkesiden. Den viser Zapier-applikationen, den konto, der skal forbindes, og de
   anmodede tilladelser (læs og skriv). Vælg **Approve**.
5. Zapier gemmer forbindelsen og mærker den med dit webstedsnavn og dit brugernavn.

Forbindelsen bruger OAuth. Ingen API-nøgle kopieres ind i Zapier, og den token, Zapier har, virker kun for
den konto, du godkendte.

## Hvem kan forbinde

Den person, der godkender forbindelsen, skal være **API-administrator** på FastComments-kontoen. Kontoejere
har denne tilladelse; andre teammedlemmer kan få den tildelt på Users-siden. En person uden denne tilladelse
ser en "du har ikke tilladelse"-side i stedet for samtykkeformularen.

## Forbindelse af det rigtige websted

Samtykkesiden forbinder den konto, du i øjeblikket er logget ind på. Hvis du administrerer flere konti,
skift til den rigtige via kontoskifteren, før du godkender, eller brug linket **switch account** på
samtykkesiden. Forbindelsesmærkaten i Zapier viser webstedsnavnet, så et forkert valg er let at opdage.

## Gennemgang og tilbagekaldelse af adgang

Enhver forbindelse vises under **Connected Apps** i FastComments-dashboardet, med de tilladelser, den har,
og hvornår den sidst blev brugt. Hvis du tilbagekalder den der, afbrydes Zapier straks; enhver Zap, der bruger
denne forbindelse, stopper, indtil den genforbindes. Du kan også fjerne forbindelsen fra Zapier-siden under
**My Apps**.