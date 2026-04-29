---
De Edit-tool stelt de agent in staat om de tekst van een bestaand commentaar te vervangen. Het is op een manier destructief die de meeste andere moderatiehulpmiddelen niet zijn: het overschrijft door gebruikers geschreven inhoud. Gebruik het alleen voor beperkte, eenduidige gevallen.

### Wat het doet

De agent geeft een commentaar-ID en een vervangende tekst door. Het platform schrijft de nieuwe tekst naar het commentaar en legt een `TextChanged` vermelding vast in het auditlogboek van het commentaar waarin zowel de oude als de nieuwe tekst worden vastgelegd. Het origineel gaat nooit verloren - moderators kunnen lezen wat het commentaar zei voordat de agent het bewerkte.

De vervangende tekst gaat door dezelfde renderingspipeline als een menselijke bewerking: vloekmaskering, parsing van vermeldingen, extractie van hashtags en afhandeling van links/afbeeldingen gedragen zich precies alsof de oorspronkelijke auteur de nieuwe tekst had ingediend.

### Reikwijdte

Zoals elk hulpmiddel dat opmerkingen wijzigt, is Edit beperkt tot de allowlist van de trigger - de agent kan alleen het commentaar bewerken waarop de trigger is geactiveerd, het oudercommentaar, of een ander binnen bereik zijnd commentaar uit dezelfde triggercontext. Een prompt-injectiepoging om "edit comment XYZ" uit te voeren waarbij XYZ niet gerelateerd is, wordt server-side geweigerd voordat de executor draait.

### Lussen

Wanneer de agent een commentaar bewerkt, vuurt het platform een `COMMENT_EDIT` trigger af zoals bij een menselijke bewerking, maar **onderdrukt het het doorsturen naar andere agenten**. Dit voorkomt dat twee agents die beide naar `COMMENT_EDIT` luisteren op elkaars bewerkingen ping-pongen.

### Wanneer toestaan

Voor agenten die PII-redactie afhandelen, of voor zelfbewerkende samenvattings-/digest-agenten. De meeste moderatieagenten hebben dit hulpmiddel **niet** nodig - mark-spam, warn, en ban dekken de typische levenscyclus.

### Goedkeuringen

**Overweeg sterk om het achter goedkeuring te plaatsen**, vooral terwijl je vertrouwen in de agent opbouwt. Een agent die de woorden van een gebruiker herschrijft is het soort actie dat een gemeenschap zal opmerken en waar men op zal reageren, en het is reputatiegewijs moeilijker te "ongedaan te maken" dan een verwijdering.

### Zie ook

- [Trigger: Comment Edited](#trigger-comment-edit) - de trigger die afgaat wanneer de tekst van een commentaar verandert.
- [Approval Workflow](#approval-workflow) - hoe je het hulpmiddel achter menselijke beoordeling kunt plaatsen.

---