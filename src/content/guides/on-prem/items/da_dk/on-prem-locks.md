Som ethvert distribueret system har FastComments brug for en måde at låse ressourcer og procedurer på. Disse låse kan overvåges via `/locks-in-progress`-endepunktet.

[For eksempel, her er endepunktet på vores US shard](https://fastcomments.com/locks-in-progress).

Dette kan være nyttigt for at se, hvorfor systemet går i stå eller er under belastning. Hvis en SRE for eksempel vil se, hvorfor systemet oplever høj CPU-belastning, kan de
tjekke dette endepunkt for at få navnet på den cron, der opfører sig forkert.