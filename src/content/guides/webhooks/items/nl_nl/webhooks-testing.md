In de Webhooks-admin zijn er `Send Test Payload`-knoppen voor elk gebeurtenistype (Create, Update, Delete). De Create- en Update-evenementen sturen een dummy WebhookComment-object, terwijl het testen van Delete een dummy request body stuurt met alleen een ID.

De test doet twee oproepen om de responsecode te verifiëren voor de "happy" (correct API Key) en "sad" (invalid API key) scenario's.

Als de test een ongeldige API key stuurt, moet je een statuscode 401 teruggeven zodat de test volledig slaagt. Als je de waarde van het token niet correct controleert, krijg je een foutmelding.

Dit is om ervoor te zorgen dat het verzoek correct wordt geauthenticeerd.