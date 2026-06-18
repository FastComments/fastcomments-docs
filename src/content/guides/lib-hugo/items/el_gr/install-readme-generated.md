---
Επιλέξτε έναν από τους δύο τυπικούς τρόπους για να προσθέσετε ένα συστατικό θέματος Hugo.

### Επιλογή A: Hugo Module (προτεινόμενο)

Από τη ρίζα του ιστότοπού σας:

```bash
hugo mod init github.com/you/your-site   # μόνο εάν ο ιστότοπός σας δεν είναι ήδη module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Στη συνέχεια προσθέστε την εισαγωγή στο `hugo.toml` σας:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Επιλογή B: Συστατικό θέματος (Git submodule)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Στη συνέχεια αναφερθείτε σ' αυτό από το `hugo.toml` σας. Καταχωρήστε το μαζί με το δικό σας θέμα — οι μεταγενέστερες εγγραφές υπερισχύουν, οπότε κρατήστε πρώτο το θέμα σας:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```
---