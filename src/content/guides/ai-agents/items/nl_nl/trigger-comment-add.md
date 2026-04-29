Activeert de agent telkens wanneer een nieuwe reactie wordt geplaatst op een pagina die binnen de agent's [scope](#scope-url-locale) valt.

### Context die de agent ontvangt

- De nieuwe reactie in zijn geheel - text, author, votes, parent ID, page URL ID.
- Optioneel: parent comment en eerdere antwoorden in dezelfde thread, als [thread context](#context-options) is ingeschakeld.
- Optioneel: de trust factor van de commenter, accountleeftijd, ban history, en recente comments, als [user history context](#context-options) is ingeschakeld.
- Optioneel: paginametadata, als [page context](#context-options) is ingeschakeld.

### Belangrijk

- De trigger vuurt **nadat** de reactie is gepersistent. De agent kan er in tool-aanroepen rechtstreeks naar verwijzen.
- Het vuurt **niet** voor reacties die zijn geschreven door een andere agent in dezelfde tenant.
- Het vuurt voor zowel geverifieerde als niet-geverifieerde reacties. Als jouw tenant moderatorapproval vereist voordat een reactie zichtbaar is (zie [Hoe goedkeuringen werken](/guide-moderation.html#moderation-approvals) in de moderatiegids), dan vuurt de trigger wanneer de reactie is aangemaakt, niet wanneer deze later wordt goedgekeurd. De moderator-bot kan worden geïnstrueerd om reacties na beoordeling voor je goed te keuren.

### Veelvoorkomende toepassingen

- **Moderatie** - controleer de reactie aan de hand van communityrichtlijnen, markeer spam of geef een waarschuwing aan eerste keer reagenaars.
- **Welkomsgroet** - hoewel [Trigger: New User First Comment](#trigger-new-user-first-comment) doorgaans beter geschikt is voor begroetingen omdat die eenmaal per gebruiker vuurt.
- **Thread summarization** - meestal gecombineerd met een [trigger delay](#trigger-deferred-delay) zodat de thread tot rust komt voordat de agent draait.