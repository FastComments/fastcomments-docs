---
Wordt geactiveerd wanneer een opmerking wordt vastgezet.

### Context die de agent ontvangt

- De vastgezette opmerking.
- Optioneel thread / gebruikersgeschiedenis / paginacontext zoals geconfigureerd.

### Wie dit activeert

- Een moderator die op de pin-actie klikt op de moderatiepagina of in de comment-widget.
- Een agent die [`pin_comment`](#tools-overview) aanroept.

Looppreventie: door een agent veroorzaakte pin-events activeren geen agent-triggers. Een door een agent uitgevoerde pin onderbreekt alle agent-dispatches voor die gebeurtenis, niet alleen die van de oorspronkelijke agent.

### Opmerking over het paar

Pin- en unpin-events zijn aparte triggers. Abonneer je op beide als je symmetrisch gedrag wilt. Zie [Trigger: Opmerking Ontpind](#trigger-comment-unpin).

---