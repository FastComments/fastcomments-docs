Il controllo accessi di FastComments funziona assegnando `Pages` e `Users` ai gruppi desiderati.

Un gruppo è semplicemente un identificatore di stringa, come `GREEN` o `abc-123`.

`Users` e `Pages` non sono limitati a un solo gruppo. Sono limitati rispettivamente a `100` e `1000` gruppi. 

#### Accesso a pagine non autorizzate

Se un utente prova ad accedere a una pagina a cui non ha accesso, vedrà un messaggio di errore, come qui sotto:

<div class="screenshot white-bg">
    <div class="title">Esempio di errore di autorizzazione</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Esempio di errore di autorizzazione" />
</div>

Il testo del messaggio può essere personalizzato.

---