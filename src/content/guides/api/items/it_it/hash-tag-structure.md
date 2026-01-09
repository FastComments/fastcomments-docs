Un oggetto `HashTag` rappresenta un tag che può essere lasciato da un utente. Gli HashTag possono essere usati per collegarsi a un contenuto esterno o per
collegare insieme commenti correlati.

La struttura dell'oggetto `HashTag` è la seguente:

[inline-code-attrs-start title = 'Struttura HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Dovrebbe iniziare con "#" o con il carattere desiderato. **/
    tag: string
    /** Un URL opzionale a cui l'hashtag può puntare. Invece di filtrare i commenti per hashtag, l'interfaccia utente verrà reindirizzata a questo al clic. **/
    url?: string
    /** SOLO LETTURA **/
    createdAt: string
}
[inline-code-end]

Note:

- In alcuni endpoint API vedrai che l'hashtag viene usato nell'URL. Ricordati di codificare i valori per URI. Per esempio, `#` dovrebbe invece essere rappresentato come `%23`.
- Alcuni di questi campi sono contrassegnati come `READONLY` - vengono restituiti dall'API ma non possono essere impostati.
 

---