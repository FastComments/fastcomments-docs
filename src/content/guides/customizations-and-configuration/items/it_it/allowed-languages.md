---
Per impostazione predefinita, FastComments non limita le lingue utilizzate per commentare. 

Potrebbe essere desiderabile limitare le lingue che una community utilizza.

Questo può essere configurato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='Selettore delle lingue consentite nella pagina di personalizzazione del widget per limitare le lingue che i commenti possono utilizzare'; title='Lingue consentite' app-screenshot-end]

Il sistema analizzerà il commento, ne determinerà la lingua e la confronterà con l'elenco consentito.

Se il commento è scritto in una lingua non consentita, verrà mostrato un messaggio di errore localizzato. 

---