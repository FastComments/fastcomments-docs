En agent har en af tre statuser:

### Disabled

Agenten er slukket. Ingen triggers behandles, og agenten vises ikke i distributionsstien. Dens kørselshistorik, analyser og hukommelse forbliver - hvis du genaktiverer den senere, er de historiske data stadig der.

Use `Disabled` when:
- Du vil tage en agent ud af rotation uden at miste den.
- En agent opfører sig dårligt, og du skal stoppe den med det samme mens du undersøger.
- Du sæsonmæssigt roterer agenter ind og ud (f.eks. en kun-holiday velkomstagent).

### Dry Run - standard for nye agenter

Agenten kører end-to-end - den behandler triggers, kalder LLM, vælger tool calls, beregner begrundelser og tillid - men **ingen reel handling udføres**. Hver kørsel registreres med **Dry Run**-mærket i [Kørselshistorik](#run-history).

Use `Dry Run` when:
- En ny agent lige er taget ud af kassen. Hvert startskabelon lander i dry-run.
- Du har redigeret prompten eller ændret trigger-sættet og vil se, hvordan ændringen spiller ud, før du forpligter dig.
- Du kører en [testkørsel / genafspilning](#test-runs-replays) (genafspilninger tvinger dry-run uanset agentens status).

Platformen opkræver tokens for dry-run-kørsler - LLM-kaldet sker stadig, kun sideeffekterne springes over. Budgetbegrænsninger gælder også for dry-run. Se [Budgetoversigt](#budgets-overview).

### Enabled

Agenten udfører reelle handlinger. Tool calls eksekveres - eller sættes i kø til [godkendelse](#approval-workflow), hvis handlingen er gated.

Use `Enabled` after dry-run output looks correct.

### Skift af status

Du kan skifte mellem enhver af de to statuser på redigeringsformularen. Skift fra Dry Run til Enabled genkører ikke retroaktivt dry-run-handlinger - disse forbliver som dry-run-historik. Nye triggers fra det øjeblik kører live.

Skift fra Enabled til Disabled midt i en kørsel afbryder **ikke** en igangværende kørsel. Den aktuelt-kørende trigger fuldfører (med hvad den allerede har startet); den næste trigger droppes, fordi agenten nu er Disabled.

### Status under betalingsproblemer

Hvis din tenants fakturering bliver ugyldig, sættes alle agenter reelt på pause uanset gemt status - triggers droppes med `BILLING_INVALID` indtil faktureringen er genoprettet. Feltet med den gemte status ændres ikke; dispatcher nægter blot at køre. Se [Planer og berettigelse](#plans-and-eligibility).