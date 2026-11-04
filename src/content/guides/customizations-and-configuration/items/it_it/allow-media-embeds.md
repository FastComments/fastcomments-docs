Per impostazione predefinita, FastComments non consente iframe nei commenti. Quando abiliti gli embed multimediali, chi commenta può incollare il codice di embed (lo snippet `<iframe>`) da provider affidabili come YouTube, Vimeo, SoundCloud e Spotify, e verrà visualizzato in linea nel commento.

Per motivi di sicurezza, non si tratta di un flag di configurazione lato client del widget. È un'impostazione lato server, validata al salvataggio di ogni commento, quindi non può essere attivata dalla pagina. Sono consentiti solo gli iframe che puntano a un elenco integrato di provider affidabili. Qualsiasi altro iframe viene rimosso.

Questa operazione si esegue senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Impostazione degli embed multimediali attivata nella pagina di personalizzazione del widget, che consente a chi commenta di incollare embed iframe affidabili'; title='Consenti embed multimediali' app-screenshot-end]

### Aggiungere i tuoi provider

Se desideri consentire embed da un provider che non è presente nell'elenco integrato di provider affidabili, aggiungi il suo hostname nel campo "Domini embed aggiuntivi" nella stessa pagina. Questi hostname sono consentiti in aggiunta ai provider integrati. La corrispondenza è esatta, quindi includi l'hostname completo (ad esempio, player.example.com). Tutto ciò che non elenchi resta bloccato.

Sia la casella di commento semplice che l'editor WYSIWYG supportano l'incollaggio di un embed. Nell'editor WYSIWYG l'embed viene inserito come blocco rimovibile.