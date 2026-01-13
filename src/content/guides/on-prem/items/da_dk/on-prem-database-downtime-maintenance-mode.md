---
FastComments understøtter en automatisk vedligeholdelsestilstand. Hvis databasen går ned, kan den fortsat levere populære kommentartråde.

Derudover gemmes i vedligeholdelsestilstand alle kommentarer i `BACKUP_DIR`. De vil blive behandlet (tjekket for spam osv.) og gemt, når systemet er online igen.

Dette gøres ved hver time at afgøre de 100 mest populære kommentartråde og cache deres indhold på disken. Afgørelsen af de 100 bedste tråde
er allerede udført ud fra forudberegnet tilstand, så det er ikke et tungt periodisk job.

Dette er fuldstændig valgfrit og aktiveres kun, hvis `CACHE_DIR` og `BACKUP_DIR` er sat. Dette gør naturligvis applikationsnoderne tilstandsholdende, men det er en tilstand, der
kan gå tabt når som helst uden at få applikationen til at opføre sig forkert.

Bemærk, at korrekt autentificering af kommentartråde ikke kan foretages i vedligeholdelsestilstand, så kun tråde, der med sikkerhed anses for offentlige, bliver periodisk sikkerhedskopieret.

I vedligeholdelsestilstand er mange funktioner ikke tilgængelige.
---