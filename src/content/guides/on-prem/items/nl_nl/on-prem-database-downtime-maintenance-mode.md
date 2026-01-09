FastComments ondersteunt een automatische onderhoudsmodus. Als de database uitvalt, kan het populaire reactiedraden blijven bedienen.

Bovendien worden in de onderhoudsmodus alle opmerkingen opgeslagen in `BACKUP_DIR`. Ze worden verwerkt (gecontroleerd op spam, enz.) en opgeslagen zodra het systeem weer online is.

Dit gebeurt doordat elk uur de top 100 meest populaire reactiedraden wordt bepaald en hun inhoud op schijf wordt gecachet. Het bepalen van de top 100 reactiedraden
wordt al gedaan op basis van vooraf berekende staat, dus het is geen zware periodieke taak.

Dit is volledig optioneel en wordt alleen ingeschakeld als `CACHE_DIR` en `BACKUP_DIR` zijn ingesteld. Dit maakt de applicatieknooppunten uiteraard stateful, maar het is staat die
op elk moment verloren kan gaan zonder dat de applicatie zich verkeerd gedraagt.

Let op dat in de onderhoudsmodus een correcte authenticatie van reactiedraden niet mogelijk is, dus worden alleen reeksen die veilig als openbaar worden beschouwd periodiek geback-upt.

In de onderhoudsmodus zijn veel functies niet beschikbaar.