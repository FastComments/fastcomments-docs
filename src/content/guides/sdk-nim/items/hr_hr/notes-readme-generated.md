### Broadcast ID-ovi

Vidjet ćete da trebate proslijediti `broadcastId` u nekim API pozivima. Kada primite događaje, vratit će vam se ovaj ID, pa ćete znati zanemariti događaj ako planirate optimistično primijeniti promjene na klijentu (što ćete vjerojatno htjeti učiniti jer pruža najbolje iskustvo). Ovdje proslijedite UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dvaput u istoj sesiji preglednika.

### SSO (Jedinstvena prijava)

Za primjere SSO-a, pogledajte dolje.