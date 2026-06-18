---
Per impostazione predefinita FastComments non consente iframe nei commenti. Quando abiliti gli embed multimediali, i commentatori possono incollare il codice embed (lo snippet `<iframe>`) da provider attendibili come YouTube, Vimeo, SoundCloud e Spotify, e verrà visualizzato inline nel commento.

Per motivi di sicurezza, questa non è un'opzione di configurazione del widget lato client. È un'impostazione lato server, convalidata quando ogni commento viene salvato, quindi non può essere attivata dalla pagina. Solo gli iframe che puntano a una lista integrata di provider attendibili sono consentiti. Qualsiasi altro iframe viene rimosso.

Questo viene fatto senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Aggiungere i propri provider

Se desideri consentire gli embed da un provider che non è presente nella lista integrata di provider attendibili, aggiungi il suo hostname nel campo "Domini aggiuntivi per gli embed" sulla stessa pagina. Questi hostnames sono consentiti oltre ai provider integrati. La corrispondenza è esatta, quindi includi l'hostname completo (ad esempio, player.example.com). Tutto ciò che non elenchi resta bloccato.

Sia la casella di commento semplice che l'editor WYSIWYG supportano l'incollamento di un embed. Nell'editor WYSIWYG l'embed viene inserito come blocco rimovibile.

---