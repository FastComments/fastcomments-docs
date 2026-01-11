I Webhooks admin er der `Send Test Payload` buttons for hver hændelsestype (Create, Update, Delete). Create- og Update-hændelserne sender et dummy WebhookComment objekt, mens test af Delete vil sende et dummy request body med kun et ID.

Testen vil lave to kald for at verificere svarstatuskoden for "happy" (korrekt API Key) og "sad" (invalid API key) scenarier.

Når testen sender en invalid API key skal du returnere statuskode 401 for at testen kan bestå fuldt ud. Hvis du ikke korrekt tjekker værdien af token, vil du få en fejl.

Dette er for at sikre, at du korrekt autentificerer anmodningen.