---
Standardmäßig begrenzt FastComments nicht die für Kommentare verwendeten Sprachen.  

Es kann wünschenswert sein, die von einer Community verwendeten Sprachen zu begrenzen.  

Dies kann ohne Code auf der Seite zur Widget‑Anpassung konfiguriert werden:  

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='Auswahl für erlaubte Sprachen auf der Widget-Anpassungsseite zum Begrenzen, welche Sprachen Kommentare verwenden dürfen'; title='Erlaubte Sprachen' app-screenshot-end]  

Das System analysiert den Kommentar, ermittelt dessen Sprache und vergleicht sie mit der erlaubten Liste.  

Wenn der Kommentar in einer nicht erlaubten Sprache verfasst ist, wird eine lokalisierte Fehlermeldung angezeigt.  

---