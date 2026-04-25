Modulet tilføjer tre Drupal-tilladelser, som du kan tildele pr. rolle under `People > Permissions`.

- **Administer FastComments** - Adgang til FastComments-indstillingsformularen på `/admin/config/content/fastcomments`.
- **View FastComments** - Påkrævet for at se kommenteringswidgetten. Uden denne tilladelse gengives widget'en ikke.
- **Toggle FastComments** - Giver brugere mulighed for at aktivere eller deaktivere kommentarer for hver enhed ved hjælp af feltets widget.

Som standard kan kun brugere med tilladelsen `administer site configuration` ændre FastComments-indstillingerne. Giv `View FastComments` til anonyme og autentificerede brugere, hvis du vil have, at besøgende skal kunne se widget'en.