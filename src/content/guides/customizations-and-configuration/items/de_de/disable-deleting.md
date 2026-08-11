---
Standardmäßig erlaubt FastComments den Benutzern, ihre Kommentare zu löschen.

Es ist jedoch möglich, dies zu verhindern.

Auf der Seite zur Widget-Anpassung sehen Sie die Option „Löschen deaktivieren“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='Option zum Deaktivieren des Löschens auf der Widget-Anpassungsseite, die verhindert, dass Kommentatoren ihre Kommentare entfernen'; title='Löschen von Kommentaren deaktivieren' app-screenshot-end]

- Dies betrifft nur reguläre Kommentatoren und nicht Moderatoren oder Administratoren, die weiterhin löschen können.
- Dies wirkt sich auch auf API-Integrationen aus, wenn `contextUserId` übergeben wird. 

---