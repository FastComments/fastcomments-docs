Som standard kan brugere slette deres egne kommentarer. Desuden medfører sletning af deres kommentar automatisk
sletning af alle underordnede og midlertidige kommentarer i tråden. Denne adfærd gælder også i live-miljøet.

Du kan begrænse dette på følgende måder:

- I stedet anonymiser den slettede kommentar (sæt navn og tekst til `[deleted]` eller en brugerdefineret værdi).
- Tillad ikke sletning af kommentarer, når der er svar. En tilpasset fejlmeddelelse vises.
- Begræns sletning, når en kommentar har svar, til kun administratorer og moderatorer.

Dette kan konfigureres via `Comment Thread Deletion`-sektionen i Widget Customization UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---