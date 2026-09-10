## Connect Your Account

1. I Zapier skal du tilføje et FastComments-trin til en Zap, eller åbne FastComments‑app‑siden i Zapier App Directory.  
2. Vælg **Sign in to FastComments**. Zapier spørger først om din region: vælg **United States**, medmindre din konto blev oprettet i EU‑regionen (`eu.fastcomments.com`).  
3. Et FastComments‑vindue åbnes. Log ind, hvis du ikke allerede er logget ind.  
4. Gennemgå samtykkesiden. Den viser Zapier‑applikationen, den konto den vil blive forbundet til, og de anmodede tilladelser (læse og skrive). Vælg **Approve**.  
5. Zapier gemmer forbindelsen og mærker den med dit webstedsnavn og brugernavn.

Forbindelsen bruger OAuth. Ingen API‑nøgle kopieres til Zapier, og tokenet som Zapier har, fungerer kun for den konto, du har godkendt.

## Who can connect

Personen, der godkender forbindelsen, skal være en **API admin** på FastComments‑kontoen. Kontoejere har denne tilladelse; andre teammedlemmer kan få den på siden Users. En person uden denne tilladelse ser en side med "du har ikke tilladelse" i stedet for samtykkesformularen.

## Connecting the right site

Samtykkesiden forbinder den konto, du er logget ind på i øjeblikket. Hvis du administrerer flere konti, skal du skifte til den rigtige fra kontoskifteren inden godkendelse, eller bruge linket **switch account** på samtykkesiden. Forbindelsesetiketten i Zapier viser webstedsnavnet, så et forkert valg er let at opdage.

## Reviewing and revoking access

Alle forbindelser vises under **Connected Apps** i FastComments‑dashboardet, med de tilladelser den har, og hvornår den sidst blev brugt. At tilbagekalde den derfra afbryder Zapier med det samme; enhver Zap, der bruger den forbindelse, stopper, indtil den genoprettes. Du kan også fjerne forbindelsen fra Zapier‑siden under **My Apps**.