---
Quando si moderano e si visualizzano i thread dei commenti, è auspicabile poter saltare direttamente a un thread per ottenere il contesto durante la moderazione.

Ciò significa che il flusso dell'utente inizia nella pagina di moderazione dei commenti, e poi dovrebbe passare da un singolo commento alla pagina che contiene quel commento, attendere il caricamento della pagina, attendere il caricamento dei commenti e poi scorrere fino a quel commento.

Tuttavia, FastComments offre un modo più rapido. Nella pagina di moderazione dei commenti, accanto a ogni commento, c'è un pulsante "Visualizza commento" nell'angolo in basso a destra.

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

Se questo commento ha risposte, il testo del pulsante indicherà invece il numero di risposte, ma cliccarlo esegue la stessa azione.

Questo pulsante ti porterà al **Visualizzatore del thread dei commenti**.

Il Visualizzatore del thread dei commenti è una piccola applicazione a caricamento rapido ospitata da FastComments che visualizza il thread di commenti per la pagina su cui si trova il commento e scorre fino a quel commento.

Questo permette ai moderatori di raccogliere rapidamente il contesto necessario, senza dover aspettare il caricamento di un'altra pagina.

---