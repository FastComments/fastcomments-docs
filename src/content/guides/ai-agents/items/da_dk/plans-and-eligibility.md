AI-agenter er tilgængelige på **Flex**- og **Pro**-planerne. Creator-planen inkluderer dem ikke.

### Plan-niveau begrænsninger

Hvert plan-niveau fastsætter:

- **Standard daglige og månedlige budgetgrænser.** Du kan sænke dem per‑agent; at hæve den per‑konto grænse kræver en plan med en højere loft. Se [Budgets Overview](#budgets-overview).

De præcise tal vises på [prissiden](https://fastcomments.com/traffic-pricing) og på din kontos faktureringsside. De vises også direkte i agentens redigeringsformular, så du aldrig behøver at forlade formularen for at finde din grænse.

FastComments Pro inkluderer $200/mo i AI‑brug. Flex faktureres med en sats på $20 per million tokens for alle modeller (i øjeblikket enten GLM 5.2 eller gpt-oss-120B).

### Fakturering skal være gyldig

AI-agenter kører kun, når lejer har **gyldig fakturering på fil**. Hvis betalingsmetoden bliver ugyldig, sættes alle agenter på pause, og AI Agents‑siden viser et banner, der instruerer den, der har **Billing Admin**‑rollen, om at opdatere faktureringen. Agenter genoptages automatisk, når faktureringen er genoprettet – ingen genafspilning eller efterfyldning af udløsere, der blev udløst under afbrydelsen.

Dette er en hård forudsætning: tokenforbrug faktureres på din konto, så platformen vil ikke sende nogen LLM‑anmodning uden en fungerende betalingsmetode.

### Hvem kan administrere agenter

Agent‑admin‑siderne er låst bag **Customization Admin**‑dashboard‑rollen. **Comment Moderator Admins** kan gennemgå og beslutte godkendelser (se [Approval Workflow](#approval-workflow)), men kan ikke oprette eller redigere agenter. **Billing Admins** modtager [budget alert emails](#budget-alerts) uanset om de har adgang til agenter.

---