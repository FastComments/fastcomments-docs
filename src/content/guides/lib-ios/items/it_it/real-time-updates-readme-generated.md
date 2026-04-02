Dopo aver chiamato `sdk.load()`, l'SDK si sottoscrive automaticamente agli eventi WebSocket per il `urlId` configurato. Vengono gestiti i seguenti eventi:

- Nuovi commenti, modifiche e eliminazioni
- Voti (nuovi e rimossi)
- Modifiche allo stato di pin, lock, flag e block
- Presenza utente (ingresso/uscita)
- Apertura/chiusura thread
- Assegnazioni di badge
- Aggiornamenti della configurazione del server

### Controllo della visualizzazione in tempo reale

Per impostazione predefinita, i nuovi commenti di altri utenti compaiono immediatamente:

```swift
sdk.showLiveRightAway = true   // Predefinito: mostra immediatamente
```

Imposta questo valore su `false` per mettere in coda i nuovi commenti dietro un pulsante "N nuovi commenti", consentendo all'utente di scegliere quando mostrarli:

```swift
sdk.showLiveRightAway = false
```

### Presenza utente

Gli indicatori online/offline compaiono automaticamente sugli avatar degli utenti quando il server abilita il tracciamento della presenza. Non è necessaria alcuna configurazione aggiuntiva sul client.

---
---