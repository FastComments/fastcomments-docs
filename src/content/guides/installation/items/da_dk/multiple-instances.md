Each instance of the comment widget is isolated. Because of this, FastComments inherently supports more than one instance per page, or multiple instances pointing to the same chat thread.

In the case of the VanillaJS library you simply have to tie the comment widget to different DOM nodes. If you want to simply update the current thread on the page, see [Skift af kommentarstråde uden at genindlæse siden](guide-customizations-and-configuration.html#switching-comment-threads);

### Synkronisering af godkendelsestilstand på tværs af flere forekomster

Lad os gennemgå eksemplet med en tilpasset single-page-applikation, der er en liste over ofte stillede spørgsmål med deres egen kommentarstråd.

I dette tilfælde har vi flere forekomster af FastComments i DOM'en på én gang.

Det er i orden, men det medfører nogle udfordringer for brugeroplevelsen.

Overvej dette forløb:

1. Brugeren besøger siden med en liste over spørgsmål, hver med deres egen kommentarswidget.  
2. Brugeren indtaster sit brugernavn og e‑mail og stiller et spørgsmål på en af trådene.  
3. De ser et andet FAQ‑element, de har et spørgsmål om.  
4. De går igen til at kommentere. Skal de indtaste deres e‑mail og brugernavn igen?

I dette tilfælde håndterer FastComments synkroniseringen af godkendelsestilstanden på tværs af widget‑forekomster for dig. I trin fire vil brugeren allerede være midlertidigt godkendt, da de indtastede deres brugernavn og e‑mail på samme side.