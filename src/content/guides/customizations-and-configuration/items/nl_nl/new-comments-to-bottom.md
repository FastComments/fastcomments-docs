[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Standaard verschijnen nieuwe live-reacties bovenaan de reactielijst zodra ze in realtime worden gepost.

Wanneer deze optie is ingeschakeld, worden nieuwe live-reacties in plaats daarvan onderaan de lijst toegevoegd. Dit beïnvloedt hoe reacties verschijnen wanneer ze live worden gepost terwijl gebruikers de reactiedraad bekijken.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Met deze instelling ingeschakeld:
- Nieuwe live-reacties geplaatst door andere gebruikers verschijnen onderaan de reactielijst
- Gebruikers zien nieuwe reacties in realtime verschijnen onder bestaande reacties
- Dit heeft alleen invloed op live-update van reacties - niet op de initiële paginalading
- Dit kan helpen de leesstroom te behouden wanneer gebruikers een discussie volgen

Let op dat deze instelling alleen van invloed is op waar nieuwe live-reacties worden geplaatst zodra ze in realtime binnenkomen. Het heeft geen invloed op de initiële sorteervolgorde wanneer de pagina wordt geladen.